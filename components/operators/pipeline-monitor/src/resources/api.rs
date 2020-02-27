use super::state::KubeResource;

use anyhow::anyhow;
use serde_json::json;

use kube::{
    api::{Api, ListParams, Object, PostParams},
    client::APIClient,
    config,
};
use k8s_openapi::api::apps::v1::{DeploymentSpec, DeploymentStatus};

fn get_resources_api() -> Api<KubeResource> {
    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = APIClient::new(kubeconfig);

    return Api::customResource(client, "resources")
        .group("minion.ponglehub.com");
}

fn get_deployments_api() -> Api<Object<DeploymentSpec, DeploymentStatus>> {
    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = APIClient::new(kubeconfig);

    return Api::v1Deployment(client)
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
    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = APIClient::new(kubeconfig);

    let deployments_api = Api::v1Deployment(client).within(namespace);

    let deployment_manifest = json!({
        "apiVersion": "apps/v1",
        "kind": "Deployment",
        "metadata": {
            "name": name,
            "labels": {
                "pipeline": pipeline,
                "resource": "resource-name"
            }
        },
        "spec": {
            "replicas": 1,
            "selector": {
                "matchLabels": {
                    "app": name
                }
            },
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
                            "image": image
                        }
                    ]
                }
            }
        }
    });

    deployments_api.create(&PostParams::default(), serde_json::to_vec(&deployment_manifest)?).await?;

    Ok(())
}