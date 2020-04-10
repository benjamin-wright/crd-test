use anyhow::anyhow;
use std::fmt;

use super::pipelines::state::{ Pipeline };
use super::resources::state::{ Resource, Secret, EnvVar };

use k8s_openapi::api::batch::v1beta1::CronJob;
use kube::api::Meta;

#[derive(Debug, Clone)]
pub struct ResourceData {
  pub image: String,
  pub name: String,
  pub resource: String,
  pub namespace: String,
  pub pipeline: String,
  pub env: Vec<EnvVar>,
  pub secrets: Vec<Secret>,
  pub resource_version: String
}

impl fmt::Display for ResourceData {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} [ns: {}, img: {}, pl: {}]", self.name, self.namespace, self.image, self.pipeline)
  }
}

#[derive(Debug)]
pub struct Operations {
  pub to_add: Vec<ResourceData>,
  pub to_update: Vec<ResourceData>,
  pub to_remove: Vec<ResourceData>
}

fn pick_resource(name: &String, resources: &Vec<Resource>) -> anyhow::Result<Resource> {
  for resource in resources {
    let resource_name = match resource.metadata.name.as_ref() {
      Some(name) => name,
      None => {
        println!("Resource missing a name in pick_resource");
        continue;
      }
    };

    if resource_name == name {
      return Ok(resource.clone());
    }
  }

  Err(anyhow!("Failed to find resource: {}", name))
}

fn get_desired_resources(pipelines: Vec<Pipeline>, resources: Vec<Resource>) -> Vec<ResourceData> {
  let mut desired_resources = vec![];

  for pipeline in pipelines {
    let pipeline_name = match pipeline.metadata.name.as_ref() {
      Some(name) => name,
      None => {
        println!("pipeline is missing a name");
        continue;
      }
    };

    let namespace = match pipeline.metadata.namespace.as_ref() {
      Some(namespace) => namespace,
      None => {
        println!("Pipeline '{}' is missing a namespace", pipeline_name);
        continue;
      },
    };

    for resource in pipeline.spec.resources {
      if !resource.trigger {
          continue;
      }

      let resource_definition = match pick_resource(&resource.name, &resources) {
        Ok(resource) => resource,
        Err(err) => {
          println!("Failed to find resource {}: {}", resource.name, err);
          continue;
        },
      };

      let resource_name = match resource_definition.metadata.name.as_ref() {
        Some(name) => name,
        None => {
          println!("Resource definition name missing for {}", resource.name);
          continue;
        }
      };

      let resource_full_name = format!("{}-{}", pipeline_name, resource_name);

      desired_resources.push(ResourceData {
        image: resource_definition.spec.image,
        name: resource_full_name,
        resource: resource_name.to_string(),
        namespace: namespace.to_string(),
        pipeline: pipeline_name.to_string(),
        env: resource_definition.spec.env,
        secrets: resource_definition.spec.secrets,
        resource_version: String::from("0")
      });
    }
  }

  return desired_resources;
}

fn get_current_resources(crons: Vec<CronJob>) -> Vec<ResourceData> {
  let mut current_resources = vec![];

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

    let annotations = match &metadata.annotations {
      Some(annotations) => annotations,
      None => {
        println!("Cron doesn't have any annotations");
        continue;
      }
    };

    let pipeline = match annotations.get("minion.ponglehub.co.uk/pipeline") {
      Some(pipeline) => pipeline,
      None => {
        println!("Cron doesn't have a pipeline annotation");
        continue;
      }
    };

    let resource = match annotations.get("minion.ponglehub.co.uk/resource") {
      Some(resource) => resource,
      None => {
        println!("Cron doesn't have a resource annotation");
        continue;
      }
    };

    let image = match annotations.get("minion.ponglehub.co.uk/image") {
      Some(image) => image,
      None => {
        println!("Cron doesn't have an image annotation");
        continue;
      }
    };

    let secrets: Vec<Secret> = match annotations.get("minion.ponglehub.co.uk/secrets") {
      Some(secrets_string) => {
        match serde_json::from_str(secrets_string) {
          Ok(secrets) => secrets,
          Err(e) => {
            println!("Error deserialising secrets string: {:?}", e);
            continue;
          }
        }
      }
      None => {
        println!("Cron doesn't have a secrets annotation");
        continue;
      }
    };

    let env: Vec<EnvVar> = match annotations.get("minion.ponglehub.co.uk/env") {
      Some(env_string) => {
        match serde_json::from_str(env_string) {
          Ok(env) => env,
          Err(e) => {
            println!("Error deserialising environment string: {:?}", e);
            continue;
          }
        }
      }
      None => {
        println!("Cron doesn't have an env annotation");
        continue;
      }
    };

    current_resources.push(ResourceData {
      image: image.to_string(),
      name: name.to_string(),
      resource: resource.to_string(),
      namespace: namespace.to_string(),
      pipeline: pipeline.to_string(),
      env: env,
      secrets: secrets,
      resource_version: Meta::resource_ver(cron).unwrap_or(String::from("0"))
    });
  }

  return current_resources;
}

pub fn get_operations(pipelines: Vec<Pipeline>, resources: Vec<Resource>, crons: Vec<CronJob>) -> Operations {
  let desired_resources = get_desired_resources(pipelines, resources);
  let current_resources = get_current_resources(crons);

  let mut to_add = vec![];
  let mut to_update = vec![];

  for resource in &desired_resources {
    let mut add = true;

    for current in &current_resources {
      if &resource.name != &current.name || &resource.namespace != &current.namespace {
        continue;
      }

      add = false;

      if &resource.pipeline != &current.pipeline {
        println!("resource monitor name {} already in use!", resource.name);
        break;
      }

      if &resource.secrets != &current.secrets || &resource.env != &current.env {
        let mut new_resource = resource.clone();
        new_resource.resource_version = current.resource_version.to_string();
        to_update.push(new_resource);
        break;
      }

      break;
    }

    if add {
      to_add.push(resource.clone());
    }
  }

  let mut to_remove = vec![];
  for current in &current_resources {
    let mut remove = true;

    for resource in &desired_resources {
      let matches = &resource.name == &current.name && &resource.namespace == &current.namespace && &resource.pipeline == &current.pipeline;

      if matches {
        remove = false;
      }
    }

    if remove {
      to_remove.push(current.clone());
    }
  }

  Operations {
    to_add,
    to_update,
    to_remove
  }
}