use std::process::Command;
use core::str::from_utf8;
extern crate ajson;
use std::{thread, time};
fn main() {



    let zone = "".to_string();
    let email = "".to_string();
    let token = "".to_string();


    let (cf_ipX, dn_list, name_list) = cf_ip(zone.clone(), email.clone(), token.clone());
    let my_ipX = my_ip();

        if cf_ipX == "failed".to_string() {

        println!("failed to curl API, check that the credentials given are correct");
        return

    }else if cf_ipX == my_ipX {

        println!("no ip change");

    }else {

        println!("ip has changed from {} to {}\nrectifying...", my_ipX, cf_ipX);
        cf_update(zone.clone(), email.clone(), token.clone(), my_ipX, dn_list, name_list);
        
    };
}


fn cf_update(zone: String, email: String, token: String, ip: String, dn_list: Vec<String>, name_id: Vec<String>) {

    let sec = time::Duration::from_secs(1);
    let email_in = "X-Auth-Email: ".to_owned() + &email;
    let token_in = "X-Auth-Key: ".to_owned() + &token;
    



            // finsih
    for x in 0..dn_list.len() {

        thread::sleep(sec);

    let zone_id = "https://api.cloudflare.com/client/v4/zones/".to_owned() + &zone + "/dns_records/" + &dn_list[x];
   
    let json = r#"
{
"type":"A","name":""#.to_owned() + &name_id[x] + r#"","content":""# + &ip + r#""
}"#;

     println!("json parse{}", json);


    

    
     let curl = Command::new("curl")
        .args([
              &zone_id,
              "-H", &email_in,
              "-H", &token_in,
              "-H", "Content-Type: application/json", "-s",
              "-X", "PUT",
              "--data",
              &json])
    .output().expect("failed to curl Cloudflare's API");

    let curl_out = from_utf8(&curl.stdout).unwrap();


    //println!("\n\n\n\n\n\nres\n{:?}", curl_out);

        };


}




fn cf_ip(zone: String, email: String, token: String) -> (String, Vec<String>, Vec<String>) {

    let zone_in = "https://api.cloudflare.com/client/v4/zones/".to_owned() + &zone + "/dns_records?";
    let email_in = "X-Auth-Email: ".to_owned() + &email;
    let token_in = "X-Auth-Key: ".to_owned() + &token;


    let curl = Command::new("curl")
        .args([
              &zone_in,
              "-H", &email_in,
              "-H", &token_in,
              "-H", "Content-Type: application/json",
              "-s"
        ]).output().expect("failed to curl Cloudflare's API");


   
    let curl_out = from_utf8(&curl.stdout).unwrap();


    //println!("{:?}", curl_out);


    let mut name_num = ajson::get(curl_out, "result.#").unwrap().to_string().parse::<i32>().unwrap();

    let mut domain_names: Vec<String> = vec![];
    
    let mut x = 0;
    for _x in 0..name_num {

        domain_names.push("<placeholder>".to_string());


        let resulter = "result.".to_owned() + &x.to_string();

        domain_names[x] = ajson::get(curl_out, &resulter).unwrap().to_string();


        x = x + 1;

        
    };

    let mut newvec: Vec<String> = vec![];
    let mut newvec2: Vec<String> = vec![];


    let mut x = 0;

    for _x in 0..domain_names.len() {

        
        let resulter = "content".to_owned() + &x.to_string();

        //println!("{}", ajson::get(&domain_names[_x], "type").unwrap().to_string());
        
        if ajson::get(&domain_names[_x], "type").unwrap().to_string() == "A" {
            newvec.push("<placeholder>".to_string());
            newvec2.push("<placeholder>".to_string());
            newvec[_x] = ajson::get(&domain_names[_x], "id").unwrap().to_string();
            newvec2[_x] = ajson::get(&domain_names[_x], "name").unwrap().to_string();
        };
    };

    println!("{:?}", newvec);
    //println!("{:?}", newvec2);

    let ip = ajson::get(curl_out, "result.0.content").unwrap().to_string();
    let dns = newvec;

    return (ip, dns, newvec2)

}


fn my_ip() -> String {

    let curl_com = Command::new("curl")
        .arg("https://api.myip.com/")
        .output()
        .expect("failed to exec");

    let curl = from_utf8(&curl_com.stdout).unwrap().to_string();
    return ajson::get(&curl, "ip").unwrap().to_string();

}
