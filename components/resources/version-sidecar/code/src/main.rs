#[macro_use] extern crate serde_derive;
#[macro_use] extern crate kube_derive;

use std::fs::File;
use std::io::prelude::*;
use anyhow::anyhow;
use std::env;

mod versions;

use versions::api::{ get_versions, add_version, VersionData };

pub struct VersionInputs {
    pipeline: String,
    resource: String,
    namespace: String
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Getting version number");

    let inputs = get_inputs()?;

    let version = get_version().await?;

    println!("Version data:");
    println!(" - namespace: {:?}", &inputs.namespace);
    println!(" - resource: {:?}", &inputs.resource);
    println!(" - pipeline: {:?}", &inputs.pipeline);
    println!(" - version: {:?}", &version);

    let versions = get_versions(&inputs.namespace).await?;
    if exists(versions, &version, &inputs) {
        println!("Version already exists!");
    } else {
        println!("Adding version...");
        add_version(&inputs.namespace, &inputs.resource, &inputs.pipeline, &version).await?;
        println!("Done");
    }

    Ok(())
}

fn exists(versions: Vec<VersionData>, version: &str, inputs: &VersionInputs) -> bool {
    for v in versions {
        let matches = v.namespace == inputs.namespace
            && v.pipeline == inputs.pipeline
            && v.resource == inputs.resource
            && v.version == version;

        if matches { return true; }
    }

    false
}

fn get_inputs() -> anyhow::Result<VersionInputs> {
    let namespace = match env::var("NAMESPACE") {
        Ok(namespace) => namespace,
        Err(_) => {
            return Err(anyhow!("Missing required environment variable NAMESPACE"));
        }
    };

    let resource = match env::var("RESOURCE") {
        Ok(resource) => resource,
        Err(_) => {
            return Err(anyhow!("Missing required environment variable RESOURCE"));
        }
    };

    let pipeline = match env::var("PIPELINE") {
        Ok(pipeline) => pipeline,
        Err(_) => {
            return Err(anyhow!("Missing required environment variable PIPELINE"));
        }
    };

    Ok(VersionInputs{
        namespace: namespace.to_string(),
        resource: resource.to_string(),
        pipeline: pipeline.to_string()
    })
}

async fn get_version() -> anyhow::Result<String> {
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
