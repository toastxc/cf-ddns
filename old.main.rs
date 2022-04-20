use error_chain::error_chain;
use std::io::Read;
// use serde_json::{Result as JSONResult, Value as JSONValue};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> reqwest::Result<()> {
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body);

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

	// let data: &str = &*body;

	// let v: JSONValue = serde_json::from_str(data);
	// println!("{}", v["origin"]);

	Ok(())
}
