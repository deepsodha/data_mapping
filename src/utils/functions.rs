use std::{collections::{HashMap, HashSet}, error::Error};

use cedar_policy::{RestrictedExpression, EntityUid};
use serde_json::Value;
use sqlx::{postgres::PgRow, Row};

use crate::{db_connectors::structs::{DBConnPool, PostgresCon, MysqlCon}, utils::{structs::{DBType, ScopeType}, traits::DataAccess}};

use super::structs::JSONConfig;

pub async fn execute_json_spec(json_data: String) -> Result<(), Box<dyn Error + Send + Sync>> {

    let mut db_conn_pool = DBConnPool::default();
    let mut attr_result_obj: HashMap<String, RestrictedExpression> = HashMap::new();
    let mut role_result_obj: HashSet<EntityUid> = HashSet::new();
    let json_config: JSONConfig = serde_json::from_str(&json_data)?;
    // println!("{json_config:?}");

    // work on attributes
    let mut attribute_future_vec_handles = Vec::with_capacity(json_config.attributes.len());
    let mut attribute_future_vec_results = Vec::with_capacity(json_config.attributes.len());

    for attr in json_config.attributes {
        println!("attr={attr:?}");
        // TODO: accept from user somehow
        let id = "user_1";

        match attr.connection {
            DBType::postgres => {

                let dbconn = match &mut db_conn_pool.postgres {
                    Some(dbconn) => {
                        println!("Already have pgconn");
                        dbconn.attribute = Some(attr);
                        dbconn.clone()
                    },
                    None => {
                        println!("Opening pgconn for the first time");
                        let mut dbconn = PostgresCon::new();
                        let pool = dbconn.open_connection().await?;
                        dbconn.pool = Some(pool);
                        dbconn.attribute = Some(attr);
                        db_conn_pool.postgres = Some(dbconn.clone());
                        dbconn
                    }
                };

                // TODO: convert attr_result to a neat JSON (HOW??)
                attribute_future_vec_handles.push(
                    tokio::spawn(async move {
                        dbconn.query_and_parse_attrs(id.to_string(), ScopeType::val).await
                    }
                ));

            },
            DBType::dynamodb => {},
            DBType::mongodb => {},
            DBType::ms_sql => {},
            DBType::mysql => {
                let dbconn = match &mut db_conn_pool.mysql {
                    Some(dbconn) => {
                        println!("Already have mysqlconn");
                        dbconn.attribute = Some(attr);
                        dbconn.clone()
                    },
                    None => {
                        println!("Opening mysqlconn for the first time");
                        let mut dbconn = MysqlCon::new();
                        let pool = dbconn.open_connection().await?;
                        dbconn.pool = Some(pool);
                        dbconn.attribute = Some(attr);
                        db_conn_pool.mysql = Some(dbconn.clone());
                        dbconn
                    }
                };

                // TODO: convert attr_result to a neat JSON (HOW??)
                attribute_future_vec_handles.push(
                    tokio::spawn(async move {
                        dbconn.query_and_parse_attrs(id.to_string(), ScopeType::val).await
                    }
                ));

            }
        }
    }

    let mut role_future_vec_handles = Vec::with_capacity(json_config.roles.len());
    let mut role_future_vec_results = Vec::with_capacity(json_config.roles.len());

    for role in json_config.roles {
        println!("role={role:?}");
        // TODO: accept from user somehow
        let id = "user_1";

        match role.connection {
            DBType::postgres => {

                let dbconn = match &mut db_conn_pool.postgres {
                    Some(dbconn) => {
                        println!("Already have mysqlconn");
                        dbconn.role = Some(role);
                        dbconn.clone()
                    },
                    None => {
                        println!("Opening mysqlconn for the first time");
                        let mut dbconn = PostgresCon::new();
                        let pool = dbconn.open_connection().await?;
                        dbconn.pool = Some(pool);
                        dbconn.role = Some(role);
                        db_conn_pool.postgres = Some(dbconn.clone());
                        dbconn
                    }
                };

                // TODO: convert attr_result to a neat JSON (HOW??)
                role_future_vec_handles.push(
                    tokio::spawn(async move {
                        dbconn.query_and_parse_roles(id.to_string(), ScopeType::val).await
                    }
                ));

            },
            DBType::dynamodb => {},
            DBType::mongodb => {},
            DBType::ms_sql => {},
            DBType::mysql => {
                
                let dbconn = match &mut db_conn_pool.mysql {
                    Some(dbconn) => {
                        println!("Already have mysqlconn");
                        dbconn.role = Some(role);
                        dbconn.clone()
                    },
                    None => {
                        println!("Opening mysqlconn for the first time");
                        let mut dbconn = MysqlCon::new();
                        let pool = dbconn.open_connection().await?;
                        dbconn.pool = Some(pool);
                        dbconn.role = Some(role);
                        db_conn_pool.mysql = Some(dbconn.clone());
                        dbconn
                    }
                };

                // TODO: convert attr_result to a neat JSON (HOW??)
                role_future_vec_handles.push(
                    tokio::spawn(async move {
                        dbconn.query_and_parse_roles(id.to_string(), ScopeType::val).await
                    }
                ));

            }
        }
    }
    
    for handle in attribute_future_vec_handles {
        attribute_future_vec_results.push(handle.await.unwrap());
    }

    for attr_result in attribute_future_vec_results {
        if attr_result.is_some() {
            attr_result_obj.extend(attr_result.unwrap());
        }
    }

    // println!("{attr_result_obj:#?}");

    
    for handle in role_future_vec_handles {
        role_future_vec_results.push(handle.await.unwrap());
    }

    for role_result in role_future_vec_results {
        if role_result.is_some() {
            role_result_obj.extend(role_result.unwrap());
        }
    }

    // println!("{role_result_obj:#?}");

    let final_ent = db_conn_pool.postgres.unwrap().merge_data(Some(attr_result_obj), role_result_obj, ScopeType::val);
    
    println!("{final_ent:#?}");

    println!("{}",final_ent.to_string());

    Ok(())
}

pub fn append_to_attr_obj(key: &String, attr_obj: &mut HashMap<String,RestrictedExpression>, v: RestrictedExpression) {
    // unwrap since append is true
    let obj_val = attr_obj.get_mut(key).unwrap();
    let old = obj_val.clone();
    *obj_val = RestrictedExpression::new_set([old,v]);
}