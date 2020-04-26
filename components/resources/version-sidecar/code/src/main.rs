use std::fs::File;
use std::io::prelude::*;
use anyhow::anyhow;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Getting version number");
    let version = get_version()?;

    println!("Version: '{}'", &version);

    Ok(())
}

fn get_version() -> anyhow::Result<String> {
    let mut file = match File::open("/output/version.txt") {
        Ok(file) => file,
        Err(err) => {
            return Err(anyhow!("Failed to find version file: {}", err));
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(err) => {
            return Err(anyhow!("Failed to read version file: {}", err));
        },
        _ => {}
    };

    let trimmed = contents.trim();
    if trimmed.len() <= 0 {
        return Err(anyhow!("Version number must be at least one character long"))
    }

    Ok(trimmed.to_string())
}