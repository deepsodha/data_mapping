use std::{error::Error, collections::{HashMap, HashSet}};
use async_trait::async_trait;
use aws_sdk_dynamodb::Client;
use cedar_policy::{RestrictedExpression, EntityUid, Entity};
use serde_json::Value;
use anyhow::Result;
use crate::utils::{traits::DataAccess, structs::{ScopeType, Attributes}, functions::append_to_attr_obj};

use super::structs::DynamodbCon;




#[async_trait]
impl DataAccess for DynamodbCon {

    type Client = Client;

    // use DBConnPool instead of opening a new conn every time?
    async fn open_connection(&self) -> Result<Self::Client, Box<dyn Error + Send + Sync>> {
        let config_data = std::fs::read_to_string("dynamodb.json")?;
        let config_data = if config_data.strip_suffix("\n").is_some() {
            config_data.strip_suffix("\n").unwrap().to_owned()
        } else {
            config_data
        };

        let conf = aws_config::load_from_env().await;
        
        let pool = aws_sdk_dynamodb::Client::new(&conf);

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