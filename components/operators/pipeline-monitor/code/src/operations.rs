use anyhow::anyhow;

use super::pipelines::state::{ Pipeline };
use super::resources::state::{ Resource };

use k8s_openapi::api::batch::v1beta1::CronJob;

#[derive(Debug)]
pub struct ResourceData {
  pub image: String,
  pub name: String,
  pub namespace: String,
  pub pipeline: String
}

#[derive(Debug)]
pub struct Operations {
  pub to_add: Vec<ResourceData>,
  pub to_update: Vec<ResourceData>,
  pub to_remove: Vec<ResourceData>
}

fn pick_resource(name: &String, resources: &Vec<Resource>) -> anyhow::Result<Resource> {
  for resource in resources {
    let resource_name = resource.metadata.name.as_ref().expect("resource missing a name");
    if resource_name == name {
      return Ok(resource.clone());
    }
  }

  Err(anyhow!("Failed to find resource: {}", name))
}

fn get_desired_resources(pipelines: Vec<Pipeline>, resources: Vec<Resource>) -> anyhow::Result<Vec<ResourceData>> {
  let mut desired_resources = vec![];

  for pipeline in pipelines {
    let pipeline_name = pipeline.metadata.name.as_ref().expect("pipeline name missing");
    let namespace = match pipeline.metadata.namespace.as_ref() {
      Some(namespace) => namespace,
      None => {
        println!("Pipeline '{}' is missing a namespace", pipeline_name);
        continue;
      },
    };

    for resource in pipeline.spec.resources {

      if !resource.trigger {
          println!(
              "Found non-triggering resource {} for pipeline '{}': {}",
              resource.name,
              namespace,
              pipeline_name
          );
          continue;
      }

      let resource_definition = match pick_resource(&resource.name, &resources) {
        Ok(resource) => resource,
        Err(err) => {
          println!("Failed to find resource {}: {}", resource.name, err);
          continue;
        },
      };

      let resource_full_name = format!("{}-{}", pipeline_name, resource_definition.metadata.name.as_ref().expect("resource definition name missing"));

      desired_resources.push(ResourceData {
        image: resource_definition.spec.image,
        name: resource_full_name,
        namespace: namespace.to_string(),
        pipeline: pipeline_name.to_string(),
      });
    }
  }

  return Ok(desired_resources);
}

pub fn get_operations(pipelines: Vec<Pipeline>, resources: Vec<Resource>, crons: Vec<CronJob>) -> Operations {
  let desired_resources = match get_desired_resources(pipelines, resources) {
    Ok(resources) => resources,
    Err(err) => {
      println!("Failed to find desired resources: ${:?}", err);
      vec![]
    }
  };

  let mut to_add = vec![];

  for resource in desired_resources {
    let mut unmonitored = true;

    for cron in &crons {
      let metadata = match &cron.metadata {
        Some(metadata) => metadata,
        None => {
          println!("Found cron without metadata");
          continue;
        }
      };

      let name = match &metadata.name {
        Some(name) => name,
        None => {
          println!("Found cron without name");
          continue;
        }
      };

      let namespace = match &metadata.namespace {
        Some(namespace) => namespace,
        None => {
          println!("Found cron without namespace");
          continue;
        }
      };

      if &resource.name == name && &resource.namespace == namespace {
        println!("resource {} is already monitored", resource.name);
        unmonitored = false;
      }
    }

    if unmonitored {
      to_add.push(resource);
    }
  }

  Operations {
    to_add,
    to_update: vec![],
    to_remove: vec![]
  }
}