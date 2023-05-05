use crate::{
    methods::driver::{result, Delta, DeltaError},
    structs::cloudflare::result::ResultDomainVec,
};

pub async fn domain_fetch(http: &Delta, zone: &str) -> Result<ResultDomainVec, DeltaError> {
    result(http.get(&format!("{zone}/dns_records")).await).await
}
