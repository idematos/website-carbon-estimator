use std::process::Command;
use serde::Deserialize;

#[derive(Deserialize)]
struct CO2Result {
    url: String,
    bytes: f64,
    cleaner_than: f64,
    statistics: Statistics,
}

#[derive(Deserialize)]
struct Statistics {
    adjusted_bytes: f64,
    energy: f64,
    co2: f64,
}

fn main() {
    let website_url = "https://www.mogrene.com"; // Change this to the URL you want to check

    let output = Command::new("node")
        .arg("co2.js")
        .arg(website_url)
        .output()
        .expect("Failed to execute node script");
    let output_str = String::from_utf8_lossy(&output.stdout);
    let co2_result: CO2Result = serde_json::from_str(&output_str).expect("Failed to parse JSON");

    println!("URL: {}", co2_result.url);
    println!("Bytes: {}", co2_result.bytes);
    println!("Cleaner than: {}%", co2_result.cleaner_than);
    println!("Adjusted Bytes: {}", co2_result.statistics.adjusted_bytes);
    println!("Energy: {} kWh", co2_result.statistics.energy);
    println!("CO2: {} g", co2_result.statistics.co2);
}
