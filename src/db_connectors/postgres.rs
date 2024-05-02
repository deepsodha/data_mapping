use std::{error::Error, collections::{HashMap, HashSet}, fmt::Display, str::FromStr};

use async_trait::async_trait;
use cedar_policy::{EntityUid, Entity, RestrictedExpression, EntityId, EntityTypeName};
use serde::Deserialize;
use serde_json::{self, Value};
use sqlx::{Pool, Postgres, PgPool, Row};
use anyhow::Result;

use crate::{utils::{traits::DataAccess, structs::{ScopeType, Attributes}, functions::append_to_attr_obj}, db_connectors::structs::PgConfigFile};
use super::NUM_CONNECTIONS;
use super::structs::{PostgresCon, PgSSLMode};

impl PostgresCon {
    pub fn new() -> PostgresCon {
        PostgresCon { pool: None, attribute: None , role: None}
    }
}



impl Display for PgSSLMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PgSSLMode::Disable => write!(f, "disable" ),
            PgSSLMode::Allow => write!(f, "allow" ),
            PgSSLMode::Prefer => write!(f, "prefer" ),
            PgSSLMode::Require => write!(f, "require" ),
            PgSSLMode::VerifyCa => write!(f, "verify-ca" ),
            PgSSLMode::VerifyFull => write!(f, "verify-full" ),
        }
    }
}




#[async_trait]
impl DataAccess for PostgresCon {

    type Client = Pool<Postgres>;

    // use DBConnPool instead of opening a new conn every time?
    async fn open_connection(&self) -> Result<Self::Client, Box<dyn Error + Send + Sync>> {
        let config = &std::fs::read_to_string("postgres.json")?;
        // println!("{config}");
        let config_data: PgConfigFile = serde_json::from_str(&config)?;
        // println!("inside open con");
        let conn_string = format!(
            "postgresql://{}:{}@{}:{}/{}?sslmode={}",
            config_data.username,
            config_data.password,
            config_data.host,
            config_data.port,
            config_data.database,
            config_data.sslmode
        );
        println!("{conn_string}");
        // let pool = PgPool::connect(&conn_string).await?;
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(NUM_CONNECTIONS)
            .connect(&conn_string).await?;

        Ok(pool)
    }

    async fn query_and_parse_attrs(
        &self,
        id: String,
        scope: ScopeType        
    ) -> Option<HashMap<String, RestrictedExpression>> {

        let mut attr_obj: HashMap<String, RestrictedExpression> = HashMap::new();
        let mut conversions: HashMap<String,String> = HashMap::new();

        match &self.attribute {
            Some(attr) => {
                // build the query
                let query = attr.query.replace("__ID__", &id).replace("__PID__", &id);

                // execute query using pool
                match &self.pool {
                    Some(pool) => {
                        
                        let rows  = sqlx::query(&query).fetch_all(pool).await.unwrap();
                        println!("PostgreSQL\n\n{} rows returned\n",rows.len());

                        for row in rows.into_iter() {
                            for (key,vals) in &attr.select_attributes {

                                println!("key={key}, vals={vals:?}\n\n");

                                for attr_props in vals.iter() {

                                    // TODO: modularize this when other dynamic functions are revealed
                                    let props_split: Vec<&str> = attr_props.split("::").collect();
                                    match props_split.get(0).unwrap().as_ref() {
                                        "Type" => {
                                            // this means `key` is of Type props_split.get(1)
                                            let key_type = props_split.get(1).unwrap().as_ref();
                                            let append = attr_obj.contains_key(key);

                                            match key_type {
                                                "String" => {
                                                    match row.try_get::<String,&str>(key.as_str()) {
                                                        Ok(v) => {
                                                            if append {
                                                                append_to_attr_obj(&key, &mut attr_obj, RestrictedExpression::new_string(v));
                                                            } else {
                                                                attr_obj.insert(key.clone(), RestrictedExpression::new_string(v));
                                                            }
                                                        },
                                                        Err(e) => {
                                                            attr_obj.insert(key.clone(), RestrictedExpression::new_string("".into()));
                                                        }
                                                    }
                                                },
                                                "Number" => {
                                                    match row.try_get::<i64,&str>(key.as_str()) {
                                                        Ok(v) => {
                                                            if append {
                                                                append_to_attr_obj(&key, &mut attr_obj, RestrictedExpression::new_long(v));
                                                            } else {
                                                                attr_obj.insert(key.clone(), RestrictedExpression::new_long(v));
                                                            }                },
                                                        Err(e) => {
                                                            attr_obj.insert(key.clone(), RestrictedExpression::new_string("".into()));
                                                        }
                                                    }
                                                },
                                                "Boolean" => {
                                                    match row.try_get::<bool,&str>(key.as_str()) {
                                                        Ok(v) => {
                                                            if append {
                                                                append_to_attr_obj(&key, &mut attr_obj, RestrictedExpression::new_bool(v));
                                                            } else {
                                                                attr_obj.insert(key.clone(), RestrictedExpression::new_bool(v));
                                                            }                },
                                                        Err(e) => {
                                                            attr_obj.insert(key.clone(), RestrictedExpression::new_string("".into()));
                                                        }
                                                    }
                                                },
                                                "Entity" => {
                                        
                                                },
                                                _ => {
                                        

                                                }
                                            }

                                            // modify_attr_map(&mut attr_obj, key_type, &row, key.clone()).await;
                                        },
                                        "!ConvertName" => {
                                            // this means `key` has to be renamed to props_split(1)
                                            let convert_to: String = props_split.get(1).unwrap().to_string();
                                            conversions.insert(key.clone(), convert_to);
                                        },
                                        _ => {}
                                    }
                                }
                            }
                        }
                    },
                    None => {
                        return None;
                    }
                }
            },
            None => {
                return  None;
            }
        }

        if conversions.len().gt(&0) {
            for (key,converted_key) in conversions {
                if attr_obj.contains_key(&key) {
                    let val = attr_obj.remove(&key).unwrap();
                    attr_obj.insert(converted_key, val);
                }
            }

        }
        
        if attr_obj.len().gt(&0) {
            // println!("\n{attr_obj:#?}");
            return Some(attr_obj);
        } else {
            return None;
        }
    }

    async fn query_and_parse_roles(&self, id: String, scope: ScopeType) -> Option<HashSet<EntityUid>> {

        let mut role_obj: HashSet<EntityUid> = HashSet::new();

        match &self.role {
            Some(role) => {
                // build the query
                let query = role.query.replace("__ID__", &id).replace("__PID__", &id);

                // execute query using pool
                match &self.pool {
                    Some(pool) => {
                        
                        let rows  = sqlx::query(&query).fetch_all(pool).await.unwrap();
                        println!("\n{} rows returned\n",rows.len());

                        for row in rows.iter() {
                            let mut role_map: HashMap<String, Value> = HashMap::new();
                            for role_props in &role.select_attributes {

                                println!("val={role_props:?}\n\n");


                                // TODO: modularize this when other dynamic functions are revealed
                                let props_split: Vec<&str> = role_props.split("::").collect();
                                match props_split.get(0).unwrap().as_ref() {
                                    "ReturnAttribute" => {
                                        let column: String = props_split.get(1).unwrap().to_string();
                                        // // cedar_policy::Entity::new(uid, attrs, parents)
                                        // let mut entity = cedar_policy::Entity::new(uid, attrs, parents)


                                        let row_value = row.try_get::<String,&str>(column.as_str()).unwrap();
                                        let euid = cedar_policy::EntityUid::from_type_name_and_id(EntityTypeName::from_str(&role.entity).unwrap(), EntityId::from_str(&row_value).unwrap());
                                        role_obj.insert(euid);
                                        // role_map.insert(String::from("type"), Value::String(role.entity.clone()));
                                        // role_map.insert(String::from("id"), Value::String(row_value));
                                    },
                                    _ => {}
                                }
                                
                            }
                            // role_obj.push(role_map);
                        }
                    },
                    None => {
                        return None;
                    }
                }
            },
            None => {
                return  None;
            }
        }

        
        if role_obj.len().gt(&0) {
            // println!("\n{role_obj:#?}");
            return Some(role_obj);
        } else {
            return None;
        }  
    }

    fn merge_data(&self, attrs: Option<HashMap<String, RestrictedExpression>>, roles: HashSet<EntityUid>, scope: ScopeType) -> Entity {
        Entity::new(
            EntityUid::from_type_name_and_id(
                EntityTypeName::from_str("User").unwrap(),
                EntityId::from_str("Dipen").unwrap()
            ),
            attrs.unwrap(), 
            roles
        )
    }
}