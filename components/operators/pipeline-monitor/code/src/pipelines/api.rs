use super::state::{ Pipeline };

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

pub async fn get_pipeline_reflector() -> anyhow::Result<Reflector<Pipeline>> {
    let client = get_api_client();
    let search_params = ListParams::default().timeout(10);
    let pipeline_resource = Resource::all::<Pipeline>();

    let pipeline_reflector: Reflector<Pipeline> = Reflector::new(client, search_params, pipeline_resource)
        .init()
        .await?;

    return Ok(pipeline_reflector);
}
