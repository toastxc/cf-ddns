pub mod cf {
    pub mod dns {
        pub mod get;
        pub mod push;
    }

    pub mod zone {
        pub mod get;
    }
}
pub mod http;

use cf::dns::get::DnsGet;
use serde::{Deserialize, Serialize};

use crate::{cf::zone::get::ZoneGet, http::Http};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IpTable {
    pub oldip: String,
    pub current: String,
    pub auth: Auth,
    pub zones: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Auth {
    email: String,
    token: String,
}

impl IpTable {
    pub fn set_old(&mut self, ip: &str) -> Self {
        self.oldip = String::from(ip);
        self.to_owned()
    }
    pub fn set_new(&mut self, ip: &str) -> Self {
        self.current = String::from(ip);
        self.to_owned()
    }
}

#[tokio::main]
async fn main() {
    let mut dir = String::new();

    for x in std::env::args() {
        let temp: Vec<&str> = x.split('=').collect();

        if temp.contains(&"dir") {
            dir = format!("{}/", temp[1].to_string());
        };
    }

    let iptable = format!("{dir}iptable.toml");
    let domain_conf = format!("{dir}domains.toml");


    println!("{iptable}");

    // if myip can be polled
    if let Ok(newip) = Http::set_ip("http://myip.wtf/text").get().await {
        // if conf file doesnt exist, fix
        if std::fs::read(&iptable).is_err() {
            let newconf = IpTable::default().set_old(&newip);
            let se_conf = toml::to_string_pretty(&newconf).unwrap();
            std::fs::write(&iptable, se_conf).unwrap();
        };

        // import and modify old conf file
        let mut conf: IpTable =
            toml::from_str(&String::from_utf8(std::fs::read(&iptable).unwrap()).unwrap()).unwrap();

        conf = conf.set_new(&newip);

        if conf.zones.is_empty() {
            // update zones
            let res = reqwest::Client::new()
                .get("https://api.cloudflare.com/client/v4/zones")
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", conf.auth.token))
                .send()
                .await
                .unwrap()
                .error_for_status();

            let mut domains: Vec<String> = Vec::new();

            // if success populate domain list
            if let Ok(response) = res {
                let value = response.json::<ZoneGet>().await.unwrap();

                for x in value.result {
                    domains.push(x.id);
                }
            };

            conf.zones = domains;

            std::fs::write(&iptable, toml::to_string_pretty(&conf).unwrap()).unwrap();
        };

        let conf = toml::from_str::<IpTable>(
            &String::from_utf8(std::fs::read(&iptable).unwrap()).unwrap(),
        )
        .unwrap();

        if conf.oldip == conf.current {
            println!("no change");
            return;
        };

        // define domains
        let mut domain_str = String::new();

        for x in conf.zones.clone() {
            let domains = domains(&conf.auth, &x).await;

            domain_str += &format!("\n\n{}", toml::to_string_pretty(&domains).unwrap());
        }

        std::fs::write(&domain_conf, domain_str).unwrap();

        // file check

        let domains = toml::from_str::<DomainHolder>(
            &String::from_utf8(std::fs::read(domain_conf).unwrap()).unwrap(),
        );
        for x in domains.unwrap().domains {
            if x.r#type == "A" {
                println!("updating {} for zone {}", x.name, x.zone);
                domain_update(&x, &conf).await;
            };
        }

        let mut newconf = conf;

        newconf.oldip = newconf.current.clone();

        std::fs::write(&iptable, toml::to_string_pretty(&newconf).unwrap()).unwrap();

        return;
    };
    println!("Could not reach myip, service unavailable");
}

async fn domain_update(domain: &Domain, conf: &IpTable) {
    // update records

    #[derive(Debug, Serialize, Deserialize)]
    struct IpUpdate {
        content: String,
        #[serde(rename = "type")]
        r#type: String,
        name: String,
        proxied: bool,
        comment: String,
        ttl: i64,
    }

    let data = IpUpdate {
        content: conf.current.clone(),
        r#type: domain.r#type.clone(),
        name: domain.name.clone(),
        proxied: domain.proxied,
        comment: domain.comment.clone(),
        ttl: domain.ttl,
    };

    let res = reqwest::Client::new()
        .put(format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
            domain.zone, domain.id
        ))
        //.headers(headermap)
        .header("Authorization", format!("Bearer {}", &conf.auth.token))
        .header("X-Auth-Email", &conf.auth.email)
        .body(serde_json::to_string(&data).unwrap())
        .send()
        .await
        .unwrap()
        .status();

    if res.is_success() {
        println!("success");
    } else {
        println!("{}", res.to_string());
    };
}

async fn domains(auth: &Auth, zone: &str) -> DomainHolder {
    let mut domains = Vec::new();
    let get = reqwest::Client::new()
        .get(format!(
            "https://api.cloudflare.com/client/v4/zones/{zone}/dns_records"
        ))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", &auth.token))
        .send()
        .await
        .unwrap()
        .error_for_status();

    if let Ok(response) = get {
        let dns = response.json::<DnsGet>().await;

        let dns = match dns {
            Ok(a) => a,
            Err(e) => {
                println!("{e}");
                panic!()
            }
        };

        for x in dns.result {
            domains.push(Domain {
                id: x.id,
                name: x.name,
                r#type: x.type_field,
                zone: zone.to_owned(),

                comment: x.comment.unwrap_or(String::from("No comment")),
                ttl: x.ttl,
                proxied: x.proxied,
            });
        }
    }
    DomainHolder { domains }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DomainHolder {
    domains: Vec<Domain>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Domain {
    id: String,
    name: String,
    r#type: String,
    zone: String,
    comment: String,
    ttl: i64,
    proxied: bool,
}
