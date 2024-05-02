use std::{error::Error, collections::{HashMap, HashSet}};
use async_trait::async_trait;
use cedar_policy::{RestrictedExpression, EntityUid, Entity};
use serde_json::Value;
use sqlx::{Pool, Mssql};
use anyhow::Result;
use crate::utils::{traits::DataAccess, structs::{ScopeType, Attributes}, functions::append_to_attr_obj};
use super::structs::MssqlCon;
use super::NUM_CONNECTIONS;





#[async_trait]
impl DataAccess for MssqlCon {

    type Client = Pool<Mssql>;

    // use DBConnPool instead of opening a new conn every time?
    async fn open_connection(&self) -> Result<Self::Client, Box<dyn Error + Send + Sync>> {
        let config_data = std::fs::read_to_string("mssql.json")?;
        let config_data = if config_data.strip_suffix("\n").is_some() {
            config_data.strip_suffix("\n").unwrap().to_owned()
        } else {
            config_data
        };

        let pool = sqlx::mssql::MssqlPoolOptions::new()
            .max_connections(NUM_CONNECTIONS)
            .connect(&config_data).await?;

        Ok(pool)
    }

    async fn query_and_parse_attrs(
        &self,
        id: String,
        scope: ScopeType        
    ) -> Option<HashMap<String, RestrictedExpression>> {
        todo!()
    }

    async fn query_and_parse_roles(&self, id: String, scope: ScopeType) -> Option<HashSet<EntityUid>> {
        todo!()
    }

    fn merge_data(&self, attrs: Option<HashMap<String, RestrictedExpression>>, roles: HashSet<EntityUid>, scope: ScopeType) -> Entity {
        todo!()
    }
}