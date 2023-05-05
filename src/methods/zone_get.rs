use crate::{
    methods::driver::{result, Delta, DeltaError},
    structs::cloudflare::result::ResultZone,
};

pub async fn zone_get(http: &Delta) -> Result<ResultZone, DeltaError> {
    result(http.get("").await).await
}
