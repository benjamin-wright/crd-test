use super::state::KubeResource;

use kube::{
    api::{Api, ListParams},
    client::APIClient,
    config,
};

fn get_resources_api() -> Api<KubeResource> {
    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = APIClient::new(kubeconfig);

    return Api::customResource(client, "resources")
        .group("minion.ponglehub.com");
}

pub async fn get_resource(name: &str) -> anyhow::Result<KubeResource> {
    let resources_api = get_resources_api();

    let resources = resources_api.list(&ListParams::default()).await?;

    for resource in &resources {
        println!("Resource: {} - Looking for {}", resource.metadata.name, name);
    }

    let resource = resources_api.get(name).await?;

    return Ok(resource);
}