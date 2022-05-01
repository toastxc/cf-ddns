use std::process::Command;
use std::io;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
use std::fs;

fn main() {

    #[derive(Serialize, Deserialize, Debug)]
    struct Cloudflare {
        create_cron: bool,
        cron_time: String,

        zone_id: String,
        record_id: String,
        record_name: String,
        user_email: String, 
        api_key: String,
        record_type: char, 
        
    }

    // user input

    println!("create cron? (true/false)");
    let mut cf_create_cron = String::new();
    io::stdin().read_line(&mut cf_create_cron)
        .ok()
        .expect("invalid input for 'create_cron',expected: bool");
    let cf_create_cron = cf_create_cron.trim().parse::<bool>().unwrap();

    println!("enter cron time (* * * * *)");
    let mut cf_cron_time = String::new();
    io::stdin().read_line(&mut cf_cron_time)
        .ok()
        .expect("invalid input for 'cron_time', expected: String");
    let cf_cron_time = cf_cron_time.trim().parse::<String>().unwrap();
    
    println!("zone_id (String)");
    let mut cf_zone_id = String::new();
    io::stdin().read_line(&mut cf_zone_id)
        .ok()
        .expect("invalid input for 'zone_id', expected: String");
    let cf_zone_id = cf_zone_id.trim().parse::<String>().unwrap();

    println!("record_id (String)");
    let mut cf_record_id = String::new();
    io::stdin().read_line(&mut cf_record_id)
        .ok()
        .expect("invalid input for 'record_id', expected: String");
    let cf_record_id = cf_record_id.trim().parse::<String>().unwrap();

    println!("record_name (String)");
    let mut cf_record_name = String::new();
    io::stdin().read_line(&mut cf_record_name)
        .ok()
        .expect("invalid input for 'record_name', expected: String");
    let cf_record_name = cf_record_name.trim().parse::<String>().unwrap();

    println!("user_email (example@email)");
    let mut cf_user_email = String::new();
    io::stdin().read_line(&mut cf_user_email)
        .ok()
        .expect("invalid input for 'user_email', expected: String");
    let cf_user_email = cf_user_email.trim().parse::<String>().unwrap();


    println!("api_key: (String)");
    let mut cf_api_key = String::new();
    io::stdin().read_line(&mut cf_api_key)
        .ok()
        .expect("invalid input for 'api_key', expected: String");
    let cf_api_key = cf_api_key.trim().parse::<String>().unwrap();


    println!("record_type (A)");
    let mut cf_record_type = String::new();
    io::stdin().read_line(&mut cf_record_type)
        .ok()
        .expect("invalid input for 'record_type', expected: Character");
    let cf_record_type = cf_record_type.trim().parse::<char>().unwrap();


    let cloudflare_json1 = Cloudflare {

        create_cron:  cf_create_cron,
        cron_time:  cf_cron_time,
        zone_id: cf_zone_id,
        record_id: cf_record_id,
        record_name: cf_record_name,
        user_email: cf_user_email,
        api_key: cf_api_key,
        record_type: cf_record_type,

    
    };

    //println!("{:#?}", cloudflare_json1);

    let cf_json = serde_json::to_string_pretty(&cloudflare_json1).unwrap();
    //println!("{:#?}", cf_json);


    let _jsoncreate = Command::new("touch")
        .arg("conf.json")
        .spawn()
        .expect("NO WIMDOES ONLY GNU/LINUX");

    fs::write("conf.json", cf_json)
        .expect("Unable to write file");

    let crontab = fs::read_to_string("/etc/crontab")
        .expect("failed to read file");
    //println!("{}", crontab);


 
    
    let crontab = crontab + &cloudflare_json1.cron_time + "    root   /home/cloudflare_api/api";

    //println!("{:#?}", crontab);



    fs::write("/etc/crontab", crontab)
        .expect("failed to write, are you running as root?");
}
