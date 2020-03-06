#[macro_use]
extern crate serde_derive;

mod pipelines;
mod resources;
mod operations;

use pipelines::api::{ get_pipeline_reflector };
use pipelines::state::{ KubePipeline };
use resources::api::{ get_resource, get_all_resources, deploy_resource_watcher };
use operations::{ Operations, get_operations };

use futures::executor;
use kube::{
    api::{WatchEvent}
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("running");

    loop {
        let pipeline_reflector = get_pipeline_reflector();
        
        match pipeline_reflector.poll() {
            Ok(_) -> refresh(pipeline_reflector),
            Err(err) -> {
                error!("Failed to refesh cache '{}' - rebooting", err);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

async fn refresh(reflector: Reflector<KubePipeline>) {
    let pipelines = pipeline_reflector.state().await?;
    let resources = get_all_resources().await?;

    let operations = get_operations(pipelines, resources);

    for resource in &operations.to_add {
        println!("adding resource: {}", resource.name);
    }

    for resource in &operations.to_update {
        println!("updating resource: {}", resource.name);
    }

    for resource in &operations.to_remove {
        println!("deleting resource: {}", resource.name);
    }
}

// async fn load_pipeline(pipeline: KubePipeline) -> anyhow::Result<()> {
//     let namespace = pipeline.metadata.namespace.as_ref().expect("Namespace not defined");
//     println!(
//         "Added a pipeline to namespace '{}': {}",
//         namespace,
//         pipeline.metadata.name
//     );

//     for resource in &pipeline.spec.resources {
//         if !resource.trigger {
//             println!(
//                 "Found non-triggering resource {} for pipeline '{}': {}",
//                 resource.name,
//                 namespace,
//                 pipeline.metadata.name
//             );

//             continue;
//         }

//         println!(
//             "Looking up resource '{}': {}",
//             namespace,
//             pipeline.metadata.name
//         );

//         let resource_definition = get_resource(&resource.name).await?;
//         let deployment_name = format!("{}-{}", pipeline.metadata.name, resource_definition.metadata.name);

//         println!(
//             "Deploying resource monitor: '{}' ({})",
//             deployment_name,
//             namespace
//         );

//         deploy_resource_watcher(
//             &deployment_name,
//             &resource_definition.spec.image,
//             &pipeline.metadata.name,
//             namespace
//         ).await?;
//     }

//     return Ok(());
// }