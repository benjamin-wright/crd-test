use super::state::KubeResource;

use anyhow::anyhow;
use serde_json::json;

use kube::{
    api::{Api, ListParams, Object, PostParams},
    client::APIClient,
    config,
};
use k8s_openapi::api::batch::v1beta1::{CronJobSpec, CronJobStatus};

fn get_resources_api() -> Api<KubeResource> {
    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = APIClient::new(kubeconfig);

    return Api::customResource(client, "resources")
        .group("minion.ponglehub.com");
}

fn get_cron_api() -> Api<Object<CronJobSpec, CronJobStatus>> {
    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = APIClient::new(kubeconfig);

    return Api::v1beta1CronJob(client)
}

pub async fn get_all_resources() -> anyhow::Result<Vec<KubeResource>> {
    let resources_api = get_resources_api();
    Ok(resources_api.list(&ListParams::default()).await?.items)
}

pub async fn get_resource(name: &str) -> anyhow::Result<KubeResource> {
    let resources_api = get_resources_api();

    let resources = resources_api.list(&ListParams::default()).await?;

    for resource in resources {
        if resource.metadata.name == name {
            return Ok(resource);
        }
    }

    return Err(anyhow!("Failed to find resource: {}", name));
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
                "resource": "resource-name"
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