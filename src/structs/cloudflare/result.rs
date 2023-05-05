use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{domain::DataDomain, zone_get::DataZone};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResultDomain {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub erros: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<DataDomain>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResultDomainVec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub erros: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<DataDomain>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResultZone {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub erros: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<Value>>,
    pub result: Vec<DataZone>,
}
