use std::{error::Error, collections::{HashMap, HashSet}};

use async_trait::async_trait;
use cedar_policy::{EntityUid, Entity, RestrictedExpression, EntityId, EntityTypeName};
use serde_json::Value;

use crate::{utils::structs::DBType, db_connectors::{structs::DBConnPool, postgres}};

use super::structs::{ScopeType, JSONConfig};


// TODO: cedar_policy
#[async_trait]
pub trait DataAccess {    
    type Client;
    
    async fn open_connection(&self) -> Result<Self::Client, Box<dyn Error + Send + Sync>>;
    async fn query_and_parse_attrs(
        &self,
        id: String,
        scope: ScopeType        
    ) -> Option<HashMap<String, RestrictedExpression>>;
    async fn query_and_parse_roles(&self, id: String, scope: ScopeType) -> Option<HashSet<EntityUid>>;
    fn merge_data(&self, attrs: Option<HashMap<String, RestrictedExpression>>, roles: HashSet<EntityUid>, scope: ScopeType) -> Entity;
}