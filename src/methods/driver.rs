use reqwest::Response;

#[derive(Debug, Clone, Default)]
pub struct Delta {
    pub url: String,
    pub token: String,
    pub timeout: std::time::Duration,
    pub headers: reqwest::header::HeaderMap,
}

impl Delta {
    pub fn new(url: &str, timeout: u64) -> Self {
        Self {
            url: String::from(url),
            timeout: std::time::Duration::from_secs(timeout),
            ..Default::default()
        }
    }
    pub fn add_header(&mut self, key: &str, value: &str) -> Self {
        let key: String = String::from(key);
        let keybytes = key.as_bytes();
        let key = reqwest::header::HeaderName::from_bytes(keybytes).unwrap();
        let value = reqwest::header::HeaderValue::from_str(value).unwrap();
        self.headers.insert(key, value);
        self.to_owned()
    }
    pub fn auth_null(&self) -> Self {
        Self {
            url: self.url.clone(),
            token: String::new(),
            timeout: self.timeout,
            headers: reqwest::header::HeaderMap::new(),
        }
    }
    pub async fn get(&self, route: &str) -> Result<Response, reqwest::Error> {
        common(
            &format!("{}{}", self.url, route),
            reqwest::Method::GET,
            None,
            self,
        )
        .await
    }
    pub async fn post(&self, route: &str, data: Option<&str>) -> Result<Response, reqwest::Error> {
        common(
            &format!("{}{}", self.url, route),
            reqwest::Method::POST,
            data,
            self,
        )
        .await
    }

    pub async fn put(&self, route: &str, data: Option<&str>) -> Result<Response, reqwest::Error> {
        common(
            &format!("{}{}", self.url, route),
            reqwest::Method::PUT,
            data,
            self,
        )
        .await
    }
}

pub async fn common(
    url: &str,
    method: reqwest::Method,
    data: Option<&str>,
    config: &Delta,
) -> Result<Response, reqwest::Error> {
    let builder = reqwest::ClientBuilder::new()
        .user_agent("TXC-CF/10.0 (Linux; async-tokio-runtime)")
        .timeout(config.timeout);

    // client constructor
    let mut client = builder.build().unwrap().request(method, url);

    // headers
    if !config.headers.is_empty() {
        client = client.headers(config.headers.clone());
    }

    // data body
    if let Some(json) = data {
        let json = json.to_string();
        client = client.header("Content-Type", "application/json").body(json);
    };

    // send request
    client.send().await
}

#[derive(Debug)]
pub enum DeltaError {
    HTTP(reqwest::StatusCode, String),
    REQWEST(reqwest::Error),
    SERDE(reqwest::Error),
}

pub async fn result<T: serde::de::DeserializeOwned>(
    http: Result<Response, reqwest::Error>,
) -> Result<T, DeltaError> {
    let res = http;
    let result: T = match res {
        Err(http) => {
            return Err(DeltaError::REQWEST(http));
        }
        Ok(a) => {
            if !a.status().is_success() {
                return Err(DeltaError::HTTP(
                    a.status(),
                    a.text().await.unwrap_or_default(),
                ));
            }
            if a.status() == 204 {
                return Ok(serde_json::from_value(serde_json::Value::Null).unwrap());
            }

            match a.json().await {
                Ok(a) => a,
                Err(a) => return Err(DeltaError::SERDE(a)),
            }
        }
    };
    Ok(result)
}
