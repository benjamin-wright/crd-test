use super::state::{ Resource as MinionResource };

use kube::{
    api::{Resource, ListParams},
    Client,
    config,
    runtime::Reflector
};

fn get_api_client() -> Client {
    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = Client::from(kubeconfig);

    return client;
}

pub async fn get_resource_reflector() -> anyhow::Result<Reflector<MinionResource>> {
    let client = get_api_client();
    let search_params = ListParams::default().timeout(10);
    let resources_resource = Resource::all::<MinionResource>();

    let resource_reflector: Reflector<MinionResource> = Reflector::new(client, search_params, resources_resource)
        .init()
        .await?;

    return Ok(resource_reflector);
}
