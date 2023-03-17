use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DnsPush {
    pub errors: Vec<Value>,
    pub messages: Vec<Value>,
    pub result: Result,
    pub success: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result {
    pub content: String,
    pub name: String,
    pub proxied: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub comment: Option<String>,
    pub created_on: String,
    pub id: String,
    pub locked: bool,
    pub meta: Meta,
    pub modified_on: String,
    pub proxiable: bool,
    pub tags: Vec<String>,
    pub ttl: i64,
    pub zone_id: String,
    pub zone_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub auto_added: bool,
    pub source: String,
}
