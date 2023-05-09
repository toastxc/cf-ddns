use std::path::PathBuf;

use cf_ddns::{
    methods::{domain_fetch, domain_update, driver::Delta, ip, zone_get},
    structs::{
        cloudflare::{domain::DataDomain, result::ResultDomainVec},
        ip::{Auth, IpTable},
        mutex::Mutex,
    },
};
use tokio::task;
#[tokio::main]
async fn main() {
    if vecer().contains(&String::from("help")) {
        println!("cf-ddns <path-to-config>");
    };

    let path = match vecer().get(1) {
        Some(arg) => PathBuf::from(arg),
        None => PathBuf::new(),
    };
    let dpath = format!("{}conf.toml", path.display());

    println!("[INFO] conf path {dpath}");

    println!("[INFO] importing config");
    let conf_raw = match std::fs::read(&dpath) {
        Ok(bytes) => String::from_utf8(bytes).unwrap(),
        Err(error) => {
            println!("[EXIT] {error}");
            return;
        }
    };

    let mut conf: IpTable = toml::from_str(&conf_raw).unwrap();
    let auth: Auth = toml::from_str(&conf_raw).unwrap();

    let mut mutex = Mutex::new().set_timeout(10).set_path(".mutex.lock");

    match mutex.gen().await {
        Ok(mut mutex) => match mutex.lock() {
            Err(mutex_lock_er) => {
                println!("[EXIT] {:#?}", mutex_lock_er);
                return;
            }
            Ok(mutex) => mutex,
        },
        Err(error) => {
            println!("[EXIT] {}", error);
            return;
        }
    };

    let client = Delta::new("https://api.cloudflare.com/client/v4/zones/", 10)
        .add_header("Authorization", &format!("Bearer {}", &auth.token))
        .add_header("X-Auth-Email", &auth.email);

    println!("[INFO] polling current address");
    if let Ok(ip) = ip::myip(client.clone()).await {
        let current_ip = ip.ip;
        if conf.ip == current_ip {
            println!("[EXIT] no ip change");
            mutex.open().unwrap();
            return;
        } else {
            println!("[MOD] updating address");
            conf.ip = current_ip;
        }

        if let Ok(zones) = zone_get::zone_get(&client).await {
            for x in zones.result {
                if !conf.zones.contains(&x.id) {
                    println!("[MOD] new zone added");
                    conf.zones.push(x.id);
                }
            }
        };
        for zone in conf.zones.clone() {
            if let Ok(ResultDomainVec {
                result: Some(domains),
                ..
            }) = domain_fetch::domain_fetch(&client, &zone).await
            {
                for mut domain in domains {
                    if domain.r#type == "A" && !conf.blacklist.contains(&domain.name) {
                        if domain.content == conf.ip.clone() {
                            println!(
                                "[INFO] content of {} is already correct, skipping",
                                domain.name
                            );
                            continue;
                        };

                        domain.content = conf.ip.clone();

                        let _ =
                            task::spawn(update_domain_status(client.clone(), domain.clone())).await;
                    }
                }
            }
        }

        let mut buffer = toml::to_string(&conf).unwrap();
        buffer += &toml::to_string(&auth).unwrap();

        println!("[INFO] procceses complete, unlocking mutex");
        mutex.open().unwrap();

        println!("[EXIT] sucessfully updated, goodbye");
        std::fs::write(&dpath, buffer).unwrap();
    } else {
        println!("[EXIT] no internet")
    }
}

pub fn vecer() -> Vec<String> {
    std::env::args().collect()
}

pub async fn update_domain_status(client: Delta, domain: DataDomain) {
    match domain_update::domain_update(client.clone(), domain.clone()).await {
        Ok(_) => println!("[MOD] updated domain {}", domain.name),
        Err(res) => println!(
            "[WARN]: failed to update domain {}\n{:#?}",
            domain.name, res
        ),
    };
}
