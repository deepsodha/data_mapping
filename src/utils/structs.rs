use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use sqlx::{postgres::PgRow, mysql::MySqlRow, Row};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScopeType {
    // TODO
    val
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DBType {
    postgres,
    mysql,
    ms_sql,
    mongodb,
    dynamodb
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExpectedRows {
    single,
    multiple
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JSONConfig {
    pub attributes: Vec<Attributes>,
    pub roles: Vec<Roles>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attributes {
    pub connection: DBType,
    pub query: String,
    pub expected_rows: ExpectedRows,
    pub select_attributes: HashMap<String, Vec<String>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roles {
    pub connection: DBType,
    pub query: String,
    pub entity: String,
    pub select_attributes: Vec<String>
}
