use crate::{
    methods::driver::{result, Delta, DeltaError},
    structs::cloudflare::{domain::DataDomain, result::ResultDomain},
};

pub async fn domain_update(http: Delta, data: DataDomain) -> Result<ResultDomain, DeltaError> {
    result(
        http.put(
            &format!("{}/dns_records/{}", data.zone_id, data.id),
            Some(&serde_json::to_string(&data).unwrap()),
        )
        .await,
    )
    .await
}
