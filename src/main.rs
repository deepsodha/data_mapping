use utils::functions::execute_json_spec;
use tracing_subscriber;

mod utils;
mod db_connectors;

use cedar_policy::PrincipalConstraint::{Any, Eq, In};
use cedar_policy::{
    Authorizer, Context, Decision, Entities, Entity, EntityId, EntityTypeName, EntityUid, Policy,
    PolicyId, PolicySet, Request, Response, RestrictedExpression, Schema, SlotId, Template,
    ValidationMode, ValidationResult, Validator,
};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

/// assumptions-
/// Connection string will be formed using a JSON file <dbtype>.json (e.g. postgres.json)
/// ID will be given at run-time, has been hardcoded for now
/// attributes expecing multiple rows are overwriting for now
/// 
/// Ready to test-
/// postgres
/// 
/// TODO-
/// other db types
/// proper tracing
/// proper error handling
/// 
#[tokio::main]
async fn main() {
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::fmt()
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .finish();

    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).unwrap();

    execute_json_spec(std::fs::read_to_string("jsonspec.json").unwrap()).await.unwrap();
    
    // let ent = create_entities_json();
    // println!("{ent:#?}");
}

// fn create_entities_json() -> Entities {
//     let e = r#"[{
//         "uid": {
//           "type": "User",
//           "id": "dipen"
//         },
//         "attrs": {
//           "firstname": "Dipen",
//           "lastname": "Javia",
//           "currency": "USD",
//           "age": 30,
//           "managers": ["john", "mike"]
//         },
//         "parents": [
//           {
//             "type": "ProductRole",
//             "id": "role1"
//           },
//           {
//             "type": "ProductRole",
//             "id": "role1"
//           }
//         ]
//       }]"#;

//       r#"
//       {
//         "attributes": [
//           {
//             "connection": "postgres",
//             "query": "select * from users where id = '__PID__'",
//             "expected_rows": "single",
//             "select_attributes": {
//               "fn": ["Type::String", "!ConvertName::firstname", "!Audit"],
//               "ln": ["Type::String", "!ConvertName::lastname", "!Audit"],
//               "currency": ["Type::String"],
//               "age": ["Type::Number"]
//             }
//           },
//           {
//             "connection": "mysql",
//             "query": "select * from org where user_id = '__PID__'",
//             "expected_rows": "multiple",
//             "select_attributes": {
//               "manager": ["Type::String", "!ConvertName::managers"]
//             }
//           }
//         ],
//         "roles": [
//           {
//             "connection": "customers_db",
//             "query": "select product from products where id = '__PID__''",
//             "entity": "ProductRole",
//             "select_attribute": ["ReturnAttribute::product"]
//           }
//         ]
      
//       "#;

//     Entities::from_json_str(e, None).expect("entity error")
// }
