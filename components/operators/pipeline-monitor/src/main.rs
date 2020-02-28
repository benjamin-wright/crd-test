#[macro_use]
extern crate serde_derive;

mod pipelines;
mod resources;
mod operations;

use pipelines::api::{ get_current_pipelines, get_pipeline_changes };
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

    prepare_state().await?;
    listen_for_changes().await?;

    Ok(())
}

async fn prepare_state() -> anyhow::Result<()> {
    let pipelines = get_current_pipelines().await?;
    let resources = get_all_resources().await?;

    let operations = get_operations();

    for resource in &operations.to_add {
        println!("adding resource: {}", resource.metadata.name);
    }

    for resource in &operations.to_update {
        println!("updating resource: {}", resource.metadata.name);
    }

    for resource in &operations.to_delete {
        println!("deleting resource: {}", resource.metadata.name);
    }

    return Ok(());
}

async fn listen_for_changes() -> anyhow::Result<()> {
    get_pipeline_changes(handle).await?;

    return Ok(())
}

fn handle(event: WatchEvent<KubePipeline>) {
    match event {
        WatchEvent::Added(pipeline) => executor::block_on(load_pipeline(pipeline)),
        WatchEvent::Modified(pipeline) => updated_pipeline(pipeline),
        WatchEvent::Deleted(pipeline) => removed_pipeline(pipeline),
        _ => error_pipeline(),
    }.expect("Failed to process pipeline event");
}

fn error_pipeline() -> anyhow::Result<()> {
    println!("another event");
    return Ok(());
}

async fn load_pipeline(pipeline: KubePipeline) -> anyhow::Result<()> {
    let namespace = pipeline.metadata.namespace.as_ref().expect("Namespace not defined");
    println!(
        "Added a pipeline to namespace '{}': {}",
        namespace,
        pipeline.metadata.name
    );

    for resource in &pipeline.spec.resources {
        if !resource.trigger {
            println!(
                "Found non-triggering resource {} for pipeline '{}': {}",
                resource.name,
                namespace,
                pipeline.metadata.name
            );

            continue;
        }

        println!(
            "Looking up resource '{}': {}",
            namespace,
            pipeline.metadata.name
        );

        let resource_definition = get_resource(&resource.name).await?;
        let deployment_name = format!("{}-{}", pipeline.metadata.name, resource_definition.metadata.name);

        println!(
            "Deploying resource monitor: '{}' ({})",
            deployment_name,
            namespace
        );

        deploy_resource_watcher(
            &deployment_name,
            &resource_definition.spec.image,
            &pipeline.metadata.name,
            namespace
        ).await?;
    }

    return Ok(());
}

fn updated_pipeline(pipeline: KubePipeline) -> anyhow::Result<()> {
    let namespace = pipeline.metadata.namespace.as_ref().expect("Namespace not defined");
    println!(
        "Updated a pipeline in namespace '{}': {}",
        namespace,
        pipeline.metadata.name
    );

    return Ok(());
}

fn removed_pipeline(pipeline: KubePipeline) -> anyhow::Result<()> {
    println!(
        "Deleted a pipeline from namespace '{}': {}",
        pipeline.metadata.namespace.as_ref().expect("Namespace not defined"),
        pipeline.metadata.name
    );

    return Ok(());
}
