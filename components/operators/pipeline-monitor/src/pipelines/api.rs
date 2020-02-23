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

pub async fn get_pipeline_changes(handler: fn(event: WatchEvent<KubePipeline>)) -> anyhow::Result<()> {
    let client = get_api_client();
    let pipelines_api = get_pipelines_api();

    // Create our informer and start listening.
    let informer = Informer::raw(client, pipelines_api)
        .init()
        .await?;

    loop {
        let mut pipeline_events = informer.poll().await?.boxed();

        // Now we just do something each time a new pipeline event is triggered.
        while let Some(event) = pipeline_events.next().await {
            handler(event?);
        }
    }
}