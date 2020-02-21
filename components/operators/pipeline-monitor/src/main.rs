#[macro_use]
extern crate serde_derive;

use serde_json::json;

use std::collections::BTreeMap;
use futures::{executor, StreamExt};
use kube::{
    api::{Api, Informer, Object, RawApi, Void, WatchEvent, Reflector, PostParams},
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
    pub resources: Vec<Resource>,
    pub steps: Vec<Step>
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

        // Now we just do something each time a new pipeline event is triggered.
        while let Some(event) = pipeline_events.next().await {
            handle(event?);
        }
    }
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

    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = APIClient::new(kubeconfig);

    let deployments = Api::v1Deployment(client).within(namespace);

    println!("{:?}", pipeline.spec);

    for resource in &pipeline.spec.resources {
        if !resource.trigger {
            println!(
                "Found non-triggering resource {} for pipeline '{}': {}",
                resource.name,
                namespace,
                pipeline.metadata.name
            );

            println!(
                "Looking up resource '{}': {}",
                namespace,
                pipeline.metadata.name
            );

            continue;
        }
    }

    let deployment_name = pipeline.metadata.name.clone() + "-" + "git-resource";
    let deployment_manifest = json!({
        "apiVersion": "apps/v1",
        "kind": "Deployment",
        "metadata": {
            "name": deployment_name,
            "labels": {
                "pipeline": pipeline.metadata.name,
                "resource": "resource-name"
            }
        },
        "spec": {
            "replicas": 1,
            "selector": {
                "matchLabels": {
                    "app": deployment_name
                }
            },
            "template": {
                "metadata": {
                    "labels": {
                        "app": deployment_name
                    }
                },
                "spec": {
                    "containers": [
                        {
                            "name": deployment_name,
                            "image": "localhost:31500/git-resource"
                        }
                    ]
                }
            }
        }
    });

    deployments.create(&PostParams::default(), serde_json::to_vec(&deployment_manifest)?).await?;

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
