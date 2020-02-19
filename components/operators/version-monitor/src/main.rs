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
pub struct Version {
    pub resource: String,
    pub pipeline: String,
    pub version: String
}

type KubeVersion = Object<Version, Void>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("running");

    prepare_state().await?;
    listen_for_changes().await?;

    Ok(())
}

fn get_versions() -> RawApi {
    return RawApi::customResource("versions")
        .group("minion.ponglehub.com");
}

async fn prepare_state() -> Result<(), anyhow::Error> {
    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = APIClient::new(kubeconfig);

    let versions_api = get_versions();

    let version_reflector: Reflector<KubeVersion> = Reflector::raw(client, versions_api)
        .timeout(10)
        .init()
        .await?;

    version_reflector.poll().await?;

    version_reflector.state().await.into_iter().for_each(|versions| {
        for version in &versions {
            println!(
                "Found Version in namespace '{}': {} -> {}:{}",
                version.metadata.namespace.as_ref().expect("Namespace not defined"),
                version.spec.pipeline,
                version.spec.resource,
                version.spec.version
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

    let versions_api = get_versions();

    // Create our informer and start listening.
    let informer = Informer::raw(client, versions_api)
        .init()
        .await?;

    loop {
        let mut version_events = informer.poll().await?.boxed();

        // Now we just do something each time a new version event is versioned.
        while let Some(event) = version_events.next().await {
            handle(event?);
        }
    }
}

fn handle(event: WatchEvent<KubeVersion>) {
    match event {
        WatchEvent::Added(version) => load_version(version),
        WatchEvent::Modified(version) => updated_version(version),
        WatchEvent::Deleted(version) => removed_version(version),
        _ => println!("another event"),
    }
}

fn load_version(version: KubeVersion) {
    println!(
        "Added a version to namespace '{}': {} -> {}:{}",
        version.metadata.namespace.as_ref().expect("Namespace not defined"),
        version.spec.pipeline,
        version.spec.resource,
        version.spec.version
    );
}

fn updated_version(version: KubeVersion) {
    println!(
        "Updated a version in namespace '{}': {} -> {}:{}",
        version.metadata.namespace.as_ref().expect("Namespace not defined"),
        version.spec.pipeline,
        version.spec.resource,
        version.spec.version
    );
}

fn removed_version(version: KubeVersion) {
    println!(
        "Deleted a version from namespace '{}': {} -> {}:{}",
        version.metadata.namespace.as_ref().expect("Namespace not defined"),
        version.spec.pipeline,
        version.spec.resource,
        version.spec.version
    );
}

async fn deploy_version(version: KubeVersion) {
    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = APIClient::new(kubeconfig);

    
}
