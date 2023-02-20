use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use url::Url;

fn main() {
    let mut url_map = HashMap::new();
    println!("URL Shortener!");
    loop {
        print!("\nEnter a url to be shortened or retrieve an url: ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = String::from(input.trim());
        if input.contains("exit") || input.trim().contains("quit") {
            break;
        }
        match Url::parse(&input) {
            Err(_) => {
                println!("Invalid url");
                continue;
            }
            Ok(url) => {
                if url.host_str() == Some("short.io") {
                    match url_map.get(url.path()) {
                        None => println!("That short URL does not exist. Please try again."),
                        Some(long) => println!("Redirecting to {}", long),
                    }
                } else {
                    let mut hasher = Sha256::new();
                    hasher.input_str(&url.path());
                    let mut short = hasher.result_str();
                    short.truncate(11);
                    short.insert_str(0, "/");
                    println!("Your shortened URL is: https://short.io{}", short);

                    if !url_map.contains_key(&short) {
                        url_map.insert(short, input);
                    }
                }
            }
        }
    }
}

fn shortener(url: &str) -> String {
    match Url::parse(&url) {
        Err(_) => panic!("Invalid url"),
        Ok(_) => {
            let mut hasher = Sha256::new();
            hasher.input_str(&url);
            hasher.result_str()
        }
    }
}
