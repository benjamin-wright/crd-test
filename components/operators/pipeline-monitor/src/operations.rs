use super::pipelines::state::{ KubePipeline };
use super::resources::state::{ KubeResource };

pub struct ResourceData {
  pub image: String,
  pub name: String
}

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

pub fn get_operations(pipelines: Vec<KubePipeline>, resources: Vec<KubeResource>) -> Operations {
  Operations::empty()
}