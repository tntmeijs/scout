use reqwest::header::USER_AGENT;
use std::error::Error;

const SCOUT_USER_AGENT: &str = "Scout/0.1";
const TARGET_URL: &str = "https://tahar.dev";

fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let raw_html = client
        .get(TARGET_URL)
        .header(USER_AGENT, SCOUT_USER_AGENT)
        .send()?
        .text()?;

    println!("{:#?}", raw_html);

    Ok(())
}
