pub mod postgres;
pub mod mysql;
pub mod mssql;
pub mod dynamodb;
pub mod mongodb;
pub mod structs;

pub const NUM_CONNECTIONS: u32 = 10;