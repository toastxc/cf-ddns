use crate::{
    methods::driver::Delta,
    structs::ip::{Auth, IpTable},
};

#[cfg(test)]
mod domain_fetch {

    use crate::methods::{domain_fetch, test::auth};

    #[tokio::test]
    pub async fn tester() {
        let (conf, client) = auth();
        let data = domain_fetch::domain_fetch(&client, &conf.zones[0]).await;

        println!("{:#?}", data);
    }
}

pub fn auth() -> (IpTable, Delta) {
    let dpath = format!("conf.toml");

    // define client
    println!("[INFO] importing config");
    let conf_raw = match std::fs::read(&dpath) {
        Ok(bytes) => String::from_utf8(bytes).unwrap(),
        Err(error) => {
            println!("[EXIT] {error}");
            panic!("conf not found for test");
        }
    };

    let conf: IpTable = toml::from_str(&conf_raw).unwrap();
    let auth: Auth = toml::from_str(&conf_raw).unwrap();

    let client = Delta::new("https://api.cloudflare.com/client/v4/zones/", 10)
        .add_header("Authorization", &format!("Bearer {}", &auth.token))
        .add_header("X-Auth-Email", &auth.email);

    (conf, client)
}
