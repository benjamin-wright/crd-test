use super::state::KubeResource;

use kube::{
    api::{RawApi, Reflector},
    client::APIClient,
    config,
};

fn get_api_client() -> APIClient {
    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = APIClient::new(kubeconfig);

    return client;
}

fn get_resources_api() -> RawApi {
    return RawApi::customResource("resources")
        .group("minion.ponglehub.com");
}

pub async fn get_resource(name: &str) -> anyhow::Result<KubeResource> {
    let client = get_api_client();
    let resources_api = get_resources_api();

    let resource = client.request::<KubeResource>(resources_api.get(name)?).await?;

    return Ok(resource);
}