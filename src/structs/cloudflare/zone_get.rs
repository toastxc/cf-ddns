use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataZone {
    pub activated_on: String,
    pub created_on: String,
    pub development_mode: i64,
    pub id: String,
    pub modified_on: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_dnshost: Option<String>,
    pub original_name_servers: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_registrar: Option<String>,
}
