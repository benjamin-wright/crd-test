use super::state::KubePipeline;

use kube::{
    api::{Informer, RawApi, Reflector, WatchEvent},
    client::APIClient,
    config,
};

use futures::StreamExt;

fn get_api_client() -> APIClient {
    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = APIClient::new(kubeconfig);

    return client;
}

fn get_pipelines_api() -> RawApi {
    return RawApi::customResource("pipelines")
        .group("minion.ponglehub.com");
}

pub async fn get_pipeline_reflector() -> Reflector<KubePipeline> {
    let client = get_api_client();
    let pipelines_api = get_pipelines_api();

    let pipeline_reflector: Reflector<KubePipeline> = Reflector::raw(client, pipelines_api)
        .timeout(10)
        .init()
        .await?;
    
    return pipeline_reflector;
}
