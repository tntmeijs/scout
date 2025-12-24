use std::error::Error;

const TARGET_URL: &str = "https://tahar.dev";

fn main() -> Result<(), Box<dyn Error>> {
    let mut webpage = webpage::webpage::Webpage::new(TARGET_URL.to_owned());
    webpage.fetch()?;
    webpage.parse()?;

    Ok(())
}
