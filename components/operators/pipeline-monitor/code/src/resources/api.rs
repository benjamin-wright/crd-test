use super::state::Resource as MinionResource;

use serde_json::json;

use kube::{
    api::{Api, DeleteParams, PostParams, Resource, ListParams},
    Client,
    config,
    runtime::Reflector
};
use k8s_openapi::api::batch::v1beta1::CronJob;

fn get_api_client() -> Client {
    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = Client::from(kubeconfig);

    return client;
}

fn get_cron_api(namespace: &str) -> Api<CronJob> {
    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = Client::from(kubeconfig);

    return Api::namespaced(client, namespace);
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

pub async fn get_resource_watch_reflector() -> anyhow::Result<Reflector<CronJob>> {
    let client = get_api_client();
    let search_params = ListParams::default().labels("minion-type=resource-watcher").timeout(10);
    let cron_resource = Resource::all::<CronJob>();

    let cron_reflector: Reflector<CronJob> = Reflector::new(client, search_params, cron_resource)
        .init()
        .await?;

    return Ok(cron_reflector);
}

pub async fn deploy_resource_watcher(name: &str, image: &str, pipeline: &str, resource: &str, namespace: &str) -> anyhow::Result<()> {
    let cron_api = get_cron_api(namespace);
    let cron_job: CronJob = serde_json::from_value(json!({
        "apiVersion": "batch/v1beta1",
        "kind": "CronJob",
        "metadata": {
            "name": name,
            "labels": {
                "pipeline": pipeline,
                "resource": resource,
                "minion-type": "resource-watcher"
            },
            "annotations": {
                "minion.ponglehub.co.uk/pipeline": pipeline,
                "minion.ponglehub.co.uk/resource": resource,
                "minion.ponglehub.co.uk/image": image,
                "minion.ponglehub.co.uk/minion-type": "resource-watcher",
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
    }))?;

    match cron_api.create(&PostParams::default(), &cron_job).await {
        Ok(_o) => return Ok(()),
        Err(kube::Error::Api(ae)) => {
            if ae.code != 409 {
                return Err(ae.into());
            }
            println!("resource monitor {} already exists", name);
            return Ok(())
        },
        Err(err) => return Err(err.into())
    }
}

pub async fn remove_resource_watcher(name: &str, namespace: &str) -> anyhow::Result<()> {
    let cron_api = get_cron_api(namespace);
    match cron_api.delete(name, &DeleteParams::default()).await {
        Ok(_o) => return Ok(()),
        Err(err) => return Err(err.into())
    }
}