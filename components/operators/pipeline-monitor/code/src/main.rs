#[macro_use] extern crate serde_derive;
#[macro_use] extern crate kube_derive;
extern crate signal_hook;

use anyhow::anyhow;
use futures_timer::Delay;
use std::time::Duration;
use signal_hook::{iterator::Signals, SIGINT, SIGTERM};

mod pipelines;
mod resources;
mod operations;

use pipelines::api::{ get_pipeline_reflector };
use pipelines::state::{ Pipeline };
use resources::api::{ get_resource_reflector, deploy_resource_watcher, remove_resource_watcher, get_resource_watch_reflector };
use resources::state::{ Resource };
use operations::{ get_operations };

use k8s_openapi::api::batch::v1beta1::CronJob;

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
        }
    });

    tokio::spawn(async move {
        loop {
            if let Err(e) = rr_cloned.poll().await {
                println!("Warning: Resource poll error: {:?}", e);
            }
        }
    });

    tokio::spawn(async move {
        loop {
            if let Err(e) = rw_cloned.poll().await {
                println!("Warning: Resource watch poll error: {:?}", e);
            }
        }
    });

    tokio::spawn(async move {
        loop {
            let pipelines = pipeline_reflector.state().await?.into_iter().collect::<Vec<_>>();
            let resources = resource_reflector.state().await?.into_iter().collect::<Vec<_>>();
            let crons = resource_watch_reflector.state().await?.into_iter().collect::<Vec<_>>();

            if let Err(e) = refresh(pipelines, resources, crons).await {
                println!("Warning: Error refreshing: {:?}", e);
            }

            Delay::new(Duration::from_secs(15)).await;
        }

        #[allow(unreachable_code)]
        Ok::<(), anyhow::Error<>>(())
    });

    let signals = match Signals::new(&[SIGINT, SIGTERM]) {
        Ok(signals) => signals,
        Err(_) => {
            return Err(anyhow!("Failed to get signals"));
        }
    };

    for sig in signals.forever() {
        println!("Exiting with signal: {:?}", sig);
        break;
    }

    Ok(())
}

async fn refresh(pipelines: Vec<Pipeline>, resources: Vec<Resource>, crons: Vec<CronJob>) -> anyhow::Result<()> {
    let operations = get_operations(pipelines, resources, crons);

    for resource in &operations.to_add {
        println!("adding resource: {}", resource);
        deploy_resource_watcher(
            &resource.name,
            &resource.image,
            &resource.pipeline,
            &resource.resource,
            &resource.namespace
        ).await?;
    }

    for resource in &operations.to_update {
        println!("updating resource: {}", resource);
    }

    for resource in &operations.to_remove {
        println!("removing resource: {}", resource);
        remove_resource_watcher(&resource.name, &resource.namespace).await?;
    }

    Ok(())
}