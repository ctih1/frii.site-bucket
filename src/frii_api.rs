use std::{time::SystemTime};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub enum DomainType {
    A,
    CNAME,
    TXT,
    NS
}

pub async fn create_domain(client: reqwest::Client, api_key: &str, domain: &str, value: &str, domain_type: DomainType, current_index: i32, max_index: i32) -> Result<(),()> {
    assert!(value.len() <= 255, "Value is too long! {}",value.len());

    let start = SystemTime::now();
    let request = client.post("https://alpha.frii.site/api/domain")
        .body(json!({
            "domain": domain,
            "value": value,
            "type": domain_type
        }).to_string())
        .header("X-API-Token", api_key)
        .header("Content-Type", "application/json")
        .send()
        .await;
    
    if let Err(error) = request {
        println!("Request to create domain {} failed! ({})", domain, error);
        return Err(());
    }
    let status_code = request.unwrap().status().as_u16();
    match &status_code {
        200 => { println!("{} uploaded ({}%) - {:.2} bytes/s", domain, (current_index as f64 / max_index as f64 * 100.0).round() as u64, (size_of_val(value) as f64 / (start.elapsed().unwrap().as_millis() as f64 / 1000.0)).round() as u64) }
        403 => { println!("Please register your BASE_DOMAIN first! Also make sure that your BASE_DOMAIN does NOT include '.frii.site'."); std::process::exit(1); }
        405 => { println!("You have hit your domain limit! (max 50)"); std::process::exit(1) }
        409 => { println!("File with that name already exists! (domain conflict)"); std::process::exit(1) }
        460 => { println!("Invalid API key!"); std::process::exit(1) }
        461 => { println!("API key missing 'register' permission!"); std::process::exit(1) }
        _ => { panic!("Unknown error ({})", status_code) }
    }

    return Ok(())
}


pub async fn get_domains(client: reqwest::Client, api_key: &str) -> serde_json::Value {
    let request = client.get("https://alpha.frii.site/api/domains")
        .header("X-API-Token", api_key)
        .send()
        .await;

    let response = request.unwrap();

    match &response.status().as_u16() {
        200 => { println!("Got details!") }
        460 => { panic!("Invalid API key!") }
        461 => { panic!("API key missing 'list' permission!") }
        _ => { panic!("Unknown error ({})", response.status().as_u16()) }
    }

    return serde_json::from_str(&response.text().await.unwrap()).unwrap();
}