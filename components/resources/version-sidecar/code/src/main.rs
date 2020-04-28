#[macro_use] extern crate serde_derive;
#[macro_use] extern crate kube_derive;

use std::fs::File;
use std::io::prelude::*;
use anyhow::anyhow;
use std::env;

mod versions;

use versions::api::{ get_versions };

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Getting version number");
    let version = get_version().await?;

    println!("Version: '{}'", &version);

    Ok(())
}

async fn get_version() -> anyhow::Result<String> {
    let namespace = match env::var("TEST_NAMESPACE") {
        Ok(namespace) => namespace,
        Err(_) => {
            return Err(anyhow!("Missing required environment variable TEST_NAMESPACE"));
        }
    };

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

    let versions = get_versions(&namespace).await?;

    println!("Versions: {:?}", versions);

    Ok(trimmed.to_string())
}
