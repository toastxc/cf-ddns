type HttpRes = Result<String, reqwest::Error>;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Http {
    pub ip: String,
}

impl Http {
    // data
    pub fn set_ip(ip: &str) -> Self {
        Self {
            ip: String::from(ip),
        }
    }

    // methods
    pub async fn get(&self) -> HttpRes {
        match reqwest::Client::new()
            .get(self.ip.clone())
            .timeout(tokio::time::Duration::from_secs(10))
            .send()
            .await
            .expect("{error}")
            .error_for_status()
        {
            Ok(a) => Ok(a.text().await.unwrap()),
            Err(a) => Err(a),
        }
    }
}
