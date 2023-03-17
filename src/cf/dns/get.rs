use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DnsGet {
    //pub errors: Vec<Value>,
    //pub messages: Vec<Value>,
    pub result: Vec<Result>,
    // pub success: bool,
    //pub result_info: ResultInfo,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResultInfo {
    pub count: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_count: i64,
}
