use aws_sdk_dynamodb::Client;
use serde::Deserialize;
use sqlx::{Pool, Mssql, MySql, PgPool, mysql::{MySqlPool, MySqlRow}, postgres::PgRow};

use crate::utils::structs::{Attributes, Roles};

#[derive(Clone, Default)]
pub struct DBConnPool {
    pub postgres: Option<PostgresCon>,
    pub mysql: Option<MysqlCon>,
    pub mssql: Option<Pool<Mssql>>,
    // pub dynamodb: Option<aws_sdk_dynamodb::Client>
    // pub mongodb
}


#[derive(Debug, Clone)]
pub struct DynamodbCon {
    pub client : Client
}

#[derive(Debug, Clone)]
pub struct MongodbCon {
    pub client : Client
}

#[derive(Debug, Clone)]
pub struct MssqlCon {
    pub pool : Pool<Mssql>
}

#[derive(Debug, Clone)]
pub struct MysqlCon {
    pub pool : Option<MySqlPool>,
    pub attribute: Option<Attributes>,
    pub role: Option<Roles>
}


#[derive(Debug, Clone)]
pub struct PostgresCon {
    pub pool : Option<PgPool>,
    pub attribute: Option<Attributes>,
    pub role: Option<Roles>
}

#[derive(Debug, Deserialize)]
pub struct PgConfigFile {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u32,
    pub database: String,
    pub sslmode: PgSSLMode
}

#[derive(Debug, Deserialize)]
pub struct MysqlConfigFile {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u32,
    pub database: String,
}

#[derive(Debug, Deserialize)]
pub enum PgSSLMode {
    #[serde(rename(deserialize = "disable"))]
    Disable,
    #[serde(rename(deserialize = "allow"))]
    Allow,
    #[serde(rename(deserialize = "prefer"))]
    Prefer,
    #[serde(rename(deserialize = "require"))]
    Require,
    #[serde(rename(deserialize = "verify-ca"))]
    VerifyCa,
    #[serde(rename(deserialize = "verify-full"))]
    VerifyFull
}