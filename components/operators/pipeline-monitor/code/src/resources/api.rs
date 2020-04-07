use super::state::{ Resource as MinionResource, EnvVar, Secret };

use serde_json::{Value, Map, json};

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

pub async fn deploy_resource_watcher(name: &str, image: &str, pipeline: &str, resource: &str, namespace: &str, env: &Vec<EnvVar>, secrets: &Vec<Secret>) -> anyhow::Result<()> {
    let cron_api = get_cron_api(namespace);

    let mut secret_mounts = vec![];
    let mut volumes = vec![];

    for secret in secrets {
        let mut secret_mount = Map::new();
        secret_mount.insert("name".to_string(), Value::String(secret.name.to_string()));
        secret_mount.insert("mountPath".to_string(), Value::String(secret.mount_path.to_string()));
        secret_mount.insert("readOnly".to_string(), Value::Bool(true));

        let mut items = vec![];
        for key_value_pair in &secret.keys {
            let mut key = Map::new();
            key.insert("key".to_string(), Value::String(key_value_pair.key.to_string()));
            key.insert("path".to_string(), Value::String(key_value_pair.path.to_string()));
            items.push(Value::Object(key));
        }

        let mut secret_hash = Map::new();
        secret_hash.insert("secretName".to_string(), Value::String(secret.name.to_string()));
        secret_hash.insert("items".to_string(), Value::Array(items));

        let mut volume = Map::new();
        volume.insert("name".to_string(), Value::String(secret.name.to_string()));
        volume.insert("secret".to_string(), Value::Object(secret_hash));

        secret_mounts.push(secret_mount);
        volumes.push(volume);
    }

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
                "minion.ponglehub.co.uk/env": json!(env).to_string(),
                "minion.ponglehub.co.uk/secrets": json!(secrets).to_string()
            }
        },
        "spec": {
            "schedule": "* * * * *",
            "concurrencyPolicy": "Forbid",
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
                                    "command": ["./version"],
                                    "env": env,
                                    "volumeMounts": secret_mounts
                                }
                            ],
                            "volumes": volumes,
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