use anyhow::anyhow;

use super::pipelines::state::{ KubePipeline };
use super::resources::state::{ KubeResource };

use kube::api::Object;
use k8s_openapi::api::batch::v1beta1::{CronJobSpec, CronJobStatus};

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

impl Operations {
  pub fn empty() -> Operations {
    Operations {
      to_add: vec![],
      to_update: vec![],
      to_remove: vec![]
    }
  }
}

fn pick_resource(name: &String, resources: &Vec<KubeResource>) -> anyhow::Result<KubeResource> {
  for resource in resources {
    if &resource.metadata.name == name {
      return Ok(resource.clone());
    }
  }

  Err(anyhow!("Failed to find resource: {}", name))
}

pub fn get_operations(pipelines: Vec<KubePipeline>, resources: Vec<KubeResource>, crons: Vec<Object<CronJobSpec, CronJobStatus>>) -> Operations {
  let mut desired_resources = vec![];

  for pipeline in pipelines {
    let namespace = match pipeline.metadata.namespace.as_ref() {
      Some(namespace) => namespace,
      None => {
        println!("Pipeline '{}' is missing a namespace", pipeline.metadata.name);
        continue;
      },
    };

    for resource in pipeline.spec.resources {
      if !resource.trigger {
          println!(
              "Found non-triggering resource {} for pipeline '{}': {}",
              resource.name,
              namespace,
              pipeline.metadata.name.clone()
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

      let resource_full_name = format!("{}-{}", pipeline.metadata.name, resource_definition.metadata.name);

      desired_resources.push(ResourceData {
        image: resource_definition.spec.image,
        name: resource_full_name,
        namespace: namespace.to_string(),
        pipeline: pipeline.metadata.name.clone(),
      });
    }
  }

  println!("{:?}", desired_resources);

  Operations::empty()
}