use super::state::KubePipeline;

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

pub fn get_pipelines_api() -> RawApi {
    return RawApi::customResource("pipelines")
        .group("minion.ponglehub.com");
}

pub async fn get_current_pipelines() -> anyhow::Result<Vec<KubePipeline>> {
    let client = get_api_client();
    let pipelines_api = get_pipelines_api();

    let pipeline_reflector: Reflector<KubePipeline> = Reflector::raw(client, pipelines_api)
        .timeout(10)
        .init()
        .await?;

    pipeline_reflector.poll().await?;

    let pipelines = pipeline_reflector.state().await?;

    return Ok(pipelines);
}