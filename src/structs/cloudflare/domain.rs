use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataDomain {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    pub content: String,
    pub created_on: String,
    pub id: String,
    pub locked: bool,
    pub meta: DataDomainMeta,
    pub modified_on: String,
    pub name: String,
    pub proxiable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Value>>,
    pub ttl: u32,
    #[serde(rename = "type")]
    pub r#type: String,
    pub zone_id: String,
    pub zone_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataDomainMeta {
    pub auto_added: bool,
    pub managed_by_apps: bool,
    pub managed_by_argo_tunnel: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
