use std::{collections::HashMap, fs::{self}, io::Write, vec};
use base64::{Engine};
use prompted::input;
use base64::engine::general_purpose::URL_SAFE;
use dotenv::dotenv;
use serde_json::{Value};
mod frii_api;

#[tokio::main]
async fn main() {
    let base_domain: String;
    let frii_site_api_key: String;

    println!("Loading..");
    dotenv().ok();

    match std::env::var("FRII_KEY") {
        Ok(val) => frii_site_api_key = val,
        Err(_) => {
            println!("Please create a .env file, and place a frii.site API key (FRII_KEY='') with the following permissions: ");
            println!("Domains: Any domain");
            println!("Permissions: Create, view domains");
            println!("Please make sure to use ' instead of \" (e.g FRII_KEY='$APIV2=...') to avoid issues with loading the API key (due to it containing $APIV2, which may be interpreted as a variable)");
            return;
        }
    }

    match std::env::var("BASE_DOMAIN") {
        Ok(val) => base_domain = val,
        Err(_) => {
            println!("Please add BASE_DOMAIN into your .env file. It should not contain the frii.site suffix (aka storage.frii.site should be stored as BASE_DOMAIN=\"storage\"");
            return;
        }
    }


    let mut file_map: HashMap<String, Vec<(u32, String)>> = HashMap::new();

    let mode = input!("Select mode ((d)ownload / (u)pload): ").to_lowercase();

    if mode == "download" || mode == "d" { 
        let domains = frii_api::get_domains(reqwest::Client::new(), &std::env::var("FRII_KEY").unwrap()).await;
        if let Value::Object(domains) = domains {
            for (domain, info) in domains {
                if !domain.contains(&base_domain) || !domain.contains("[dot]") {
                    continue;
                }

                let parts: Vec<String> = domain.split("[dot]").map(|s| s.to_string()).collect();
                let number: u32 = parts.first().unwrap()[1..parts.first().unwrap().len()].parse().unwrap();

                file_map.entry(parts[1..parts.len()-1].join("."))
                    .or_insert(vec![])
                    .push(
                        (number, info.get("ip")
                        .unwrap()
                        .to_string()
                        .replace("\"", "")
                        .replace("\\","")
                    )
                );
            }
        }

        for (file, mut content) in file_map.into_iter() {
            content.sort_by(|(i,_), (i2, _)| i.cmp(i2));
            println!("Decoding {} parts", &content.len());
            let base64 = content.iter()
                .map(|(_index, part)| 
                    part.to_string()
                ).collect::<Vec<String>>()
                .join("");
            
            // .unwrap().write_all(&URL_SAFE.decode(base64).unwrap());
            let file_handle = fs::OpenOptions::new().write(true).create(true).open(&file);
            
            if let Err(err) = file_handle {
                println!("Failed to open file {}! ({}), skipping", &file, err);
                continue;
            }

            let decoded_bytes = URL_SAFE.decode(base64);

            if let Err(err) = decoded_bytes {
                println!("Failed to decode file {}! ({}), skipping", &file, err);
                continue;
            }

            if let Err(err) = file_handle.unwrap().write_all(&decoded_bytes.unwrap()) {
                print!("Failed to write file {}! ({}), skipping", &file, err);
                continue;
            }

            println!("Saved {}", &file);
        }
        return;
    } else if mode == "u" || mode == "upload" {
        let path = input!("Enter path to your file: ");
            let filename = input!("Enter your desired filename (e.g test.txt): ");

            let file_bytes = fs::read(path).unwrap();

            println!("File bytes: {}", file_bytes.len());

            let base64_string: String = URL_SAFE.encode(file_bytes);
            let mut base64_parts: Vec<String> = vec![];

            for chunk in base64_string.as_bytes().chunks(250) {
                base64_parts.push(String::from_utf8(chunk.to_vec()).unwrap());
            }

            println!("{} subdomains needed to store data..",base64_parts.len());

            if base64_parts.len() > 50 {
                println!("WARNING: Default subdomain domain limit is 50");
                if input!("Do you want to continue? <y/n>") != "y" {
                    println!("Quitting...");
                    return
                }
            }

            println!("Starting creation process...");

            for (index, part) in base64_parts.iter().enumerate() {
                let _ = frii_api::create_domain(
                    reqwest::Client::new(),
                    &frii_site_api_key,
                    &format!("p{}.{}.{}", index, filename, base_domain),
                    &format!("\"{}\"",part),
                    frii_api::DomainType::TXT,
                    index as i32,
                    base64_parts.len() as i32
                ).await;
            }
    } else {
        println!("Invalid mode! Pleas select 'upload' or 'download'!");
        return;
    }

}
