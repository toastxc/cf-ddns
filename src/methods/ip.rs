use crate::methods::driver::{result, Delta, DeltaError};
use serde::{Deserialize, Serialize};

pub async fn myip(mut http: Delta) -> Result<DataIpFetch, DeltaError> {
    http.url = String::from("https://myip.wtf/json");

    result(http.auth_null().get("").await).await
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataIpFetch {
    #[serde(rename = "YourFuckingIPAddress")]
    pub ip: String,
}
