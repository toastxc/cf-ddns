use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZoneGet {
    pub errors: Vec<Value>,
    pub messages: Vec<Value>,
    pub success: bool,
    pub result_info: ResultInfo,
    pub result: Vec<Result>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResultInfo {
    pub count: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result {
    pub activated_on: String,
    pub created_on: String,
    pub development_mode: i64,
    pub id: String,
    pub modified_on: String,
    pub name: String,
    pub original_dnshost: String,
    pub original_name_servers: Vec<String>,
    pub original_registrar: String,
}
