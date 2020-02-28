mod pipelines;
mod resources;

use pipelines::state::{ KubePipeline };
use resources::state::{ KubeResource };

pub struct Operations {
  to_add: Vec<KubeResource>,
  to_update: Vec<KubeResource>
  to_remove: Vec<KubeResource>
}

pub fn get_operations(pipelines: Vec<KubePipeline>, resources: Vec<KubeResource>) -> Operations {
  return Operations {
    ToAdd: vec![]
    ToUpdate: vec![]
    ToRemove: vec![]
  }
}