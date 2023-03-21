extern crate regex;

use regex::Regex;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input from stdin");

    let addresses = extract_ethereum_addresses(&input);
    println!("[");
    for address in addresses {
        println!("\"{}\",", address);
    }
    println!("]");
}

fn extract_ethereum_addresses(input: &str) -> Vec<&str> {
    let re = Regex::new(r"(?i)0x[a-fA-F0-9]{40}").unwrap();
    let addresses: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();
    addresses
}
