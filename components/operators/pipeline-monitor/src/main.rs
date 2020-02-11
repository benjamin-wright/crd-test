#[macro_use]
extern crate serde_derive;

use std::collections::BTreeMap;
use futures::StreamExt;
use kube::{
    api::{Informer, Object, RawApi, Void, WatchEvent, Reflector},
    client::APIClient,
    config,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SecretKey {
    pub key: String,
    pub path: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Secret {
    pub name: String,
    #[serde(flatten)]
    pub keys: Option<Vec<SecretKey>>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Resource {
    pub name: String,
    pub trigger: bool,
    #[serde(flatten)]
    pub secrets: Option<Vec<Secret>>,
    pub env: BTreeMap<String, String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Step {
    pub name: String,
    pub resource: Option<String>,
    pub action: Option<String>,
    pub path: Option<String>,
    pub image: Option<String>,
    pub command: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pipeline {
    #[serde(flatten)]
    pub resources: Option<Vec<Resource>>,
    #[serde(flatten)]
    pub steps: Option<Vec<Step>>
}

type KubePipeline = Object<Pipeline, Void>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("running");

    prepare_state().await?;
    listen_for_changes().await?;

    Ok(())
}

fn get_pipelines() -> RawApi {
    return RawApi::customResource("pipelines")
        .group("minion.ponglehub.com");
}

async fn prepare_state() -> Result<(), anyhow::Error> {
    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = APIClient::new(kubeconfig);

    let pipelines_api = get_pipelines();

    let pipeline_reflector: Reflector<KubePipeline> = Reflector::raw(client, pipelines_api)
        .timeout(10)
        .init()
        .await?;

    pipeline_reflector.poll().await?;

    pipeline_reflector.state().await.into_iter().for_each(|pipelines| {
        for pipeline in &pipelines {
            println!(
                "Found Pipeline in namespace '{}': {}",
                pipeline.metadata.namespace.as_ref().expect("Namespace not defined"),
                pipeline.metadata.name
            );
        }
    });

    return Ok(());
}

async fn listen_for_changes() -> anyhow::Result<()> {
    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = APIClient::new(kubeconfig);

    let pipelines_api = get_pipelines();

    // Create our informer and start listening.
    let informer = Informer::raw(client, pipelines_api)
        .init()
        .await?;

    loop {
        let mut pipeline_events = informer.poll().await?.boxed();

        // Now we just do something each time a new task event is triggered.
        while let Some(event) = pipeline_events.next().await {
            handle(event?);
        }
    }
}

fn handle(event: WatchEvent<KubePipeline>) {
    match event {
        WatchEvent::Added(pipeline) => load_pipeline(pipeline),
        WatchEvent::Deleted(pipeline) => removed_pipeline(pipeline),
        _ => println!("another event"),
    }
}

fn load_pipeline(pipeline: KubePipeline) {
    println!(
        "Added a pipeline to namespace '{}': {}",
        pipeline.metadata.namespace.as_ref().expect("Namespace not defined"),
        pipeline.metadata.name
    );
}

fn removed_pipeline(pipeline: KubePipeline) {
    println!(
        "Deleted a pipeline from namespace '{}': {}",
        pipeline.metadata.namespace.as_ref().expect("Namespace not defined"),
        pipeline.metadata.name
    );
}
