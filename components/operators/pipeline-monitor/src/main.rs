#[macro_use]
extern crate serde_derive;

use futures_timer::Delay;
use std::time::Duration;

mod pipelines;
mod resources;
mod operations;

use pipelines::api::{ get_pipeline_reflector };
use pipelines::state::{ KubePipeline };
use resources::api::{ get_resource_reflector, deploy_resource_watcher, get_resource_watch_reflector };
use resources::state::{ KubeResource };
use operations::{ get_operations };


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("running");

    let pipeline_reflector = get_pipeline_reflector().await?;
    let resource_reflector = get_resource_reflector().await?;
    let resource_watch_reflector = get_resource_watch_reflector().await?;

    let pr_cloned = pipeline_reflector.clone();
    let rr_cloned = resource_reflector.clone();
    let rw_cloned = resource_watch_reflector.clone();

    tokio::spawn(async move {
        loop {
            if let Err(e) = pr_cloned.poll().await {
                println!("Warning: Pipeline poll error: {:?}", e);
            }

            if let Err(e) = rr_cloned.poll().await {
                println!("Warning: Resource poll error: {:?}", e);
            }

            if let Err(e) = rw_cloned.poll().await {
                println!("Warning: Resource watch poll error: {:?}", e);
            }
        }
    });

    loop {
        Delay::new(Duration::from_secs(5)).await;

        let pipelines = pipeline_reflector.state().await?.into_iter().collect::<Vec<_>>();
        let resources = resource_reflector.state().await?.into_iter().collect::<Vec<_>>();
        let crons = resource_watch_reflector.state().await?.into_iter().collect::<Vec<_>>();

        refresh(pipelines, resources, crons).await?;
    }
}

async fn refresh(pipelines: Vec<KubePipeline>, resources: Vec<KubeResource>, crons: Vec<Object<CronJobSpec, CronJobStatus>>) -> anyhow::Result<()> {
    let operations = get_operations(pipelines, resources, crons);

    for resource in &operations.to_add {
        println!("adding resource: {}", resource.name);
        deploy_resource_watcher(
            &resource.name,
            &resource.image,
            &resource.pipeline,
            &resource.namespace
        ).await?;
    }

    for resource in &operations.to_update {
        println!("updating resource: {}", resource.name);
    }

    for resource in &operations.to_remove {
        println!("deleting resource: {}", resource.name);
    }

    Ok(())
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