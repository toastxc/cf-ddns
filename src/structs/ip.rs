use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IpUpdate {
    pub content: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub name: String,
    pub proxied: bool,
    pub comment: String,
    pub ttl: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Auth {
    pub email: String,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IpTable {
    pub ip: String,
    pub zones: Vec<String>,
    pub blacklist: Vec<String>,
}