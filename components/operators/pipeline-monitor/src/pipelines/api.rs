use super::state::KubePipeline;

use kube::{
    api::{Resource},
    Client,
    config,
    runtime::Reflector
};

fn get_api_client() -> Client {
    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = Client::new(kubeconfig);

    return client;
}

fn get_pipelines_api() -> Resource {
    return Resource::customResource("pipelines")
        .group("minion.ponglehub.com");
}

pub async fn get_pipeline_reflector() -> anyhow::Result<Reflector<KubePipeline>> {
    let client = get_api_client();
    let pipelines_api = get_pipelines_api();

    let pipeline_reflector: Reflector<KubePipeline> = Reflector::raw(client, pipelines_api)
        .timeout(10)
        .init()
        .await?;

    return Ok(pipeline_reflector);
}
