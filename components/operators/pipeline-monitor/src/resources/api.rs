use super::state::KubeResource;

use serde_json::json;

use kube::{
    api::{Api, Object, PostParams, Resource, ListParams},
    Client,
    config,
    runtime::Reflector
};
use k8s_openapi::api::batch::v1beta1::{CronJobSpec, CronJobStatus};

fn get_api_client() -> Client {
    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = Client::new(kubeconfig);

    return client;
}

fn get_resources_api() -> Resource {
    return Resource::customResource("resources")
        .group("minion.ponglehub.com");
}

fn get_cron_api() -> Api<Object<CronJobSpec, CronJobStatus>> {
    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = Client::new(kubeconfig);

    return Api::v1beta1CronJob(client)
}

pub async fn get_resource_reflector() -> anyhow::Result<Reflector<KubeResource>> {
    let client = get_api_client();
    let resources_api = get_resources_api();

    let resource_reflector: Reflector<KubeResource> = Reflector::raw(client, resources_api)
        .timeout(10)
        .init()
        .await?;

    return Ok(resource_reflector);
}

pub async fn get_resource_watch_reflector() -> anyhow::Result<Reflector<Object<CronJobSpec, CronJobStatus>>> {
    let client = get_api_client();
    let search_params = ListParams::default().labels("minion.ponglehub.co.uk/minion-type=resource-watcher");
    let cron_resource = Resource::all::<Object<CronJobSpec, CronJobStatus>>();

    let cron_reflector: Reflector<Object<CronJobSpec, CronJobStatus>> = Reflector::new(client, search_params, cron_resource)
        .timeout(10)
        .init()
        .await?;

    return Ok(cron_reflector);
}

pub async fn deploy_resource_watcher(name: &str, image: &str, pipeline: &str, namespace: &str) -> anyhow::Result<()> {
    let cron_api = get_cron_api();
    let job_manifest = json!({
        "apiVersion": "batch/v1beta1",
        "kind": "CronJob",
        "metadata": {
            "name": name,
            "labels": {
                "pipeline": pipeline,
                "resource": "resource-name",
                "minion-type": "resource-watcher"
            }
        },
        "spec": {
            "schedule": "* * * * *",
            "jobTemplate": {
                "spec": {
                    "template": {
                        "metadata": {
                            "labels": {
                                "app": name
                            }
                        },
                        "spec": {
                            "containers": [
                                {
                                    "name": name,
                                    "image": image,
                                    "command": ["./version"]
                                }
                            ],
                            "restartPolicy": "Never"
                        }
                    }
                }
            }
        }
    });

    cron_api.within(namespace).create(&PostParams::default(), serde_json::to_vec(&job_manifest)?).await?;

    Ok(())
}