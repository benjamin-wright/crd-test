use super::state::{ Version };

use serde_json::json;

use kube::{
    api::{Api, ListParams, PostParams},
    Client,
    config
};

fn get_version_body(resource: &str, pipeline: &str, version: &str) -> anyhow::Result<Version> {
    let version: Version = serde_json::from_value(json!({
        "apiVersion": "minion.ponglehub.com/v1",
        "kind": "Version",
        "metadata": {
            "name": format!("{}-{}-{}", resource, pipeline, version)
        },
        "spec": {
            "resource": resource,
            "pipeline": pipeline,
            "version": version
        }
    }))?;

    return Ok(version);
}

fn get_api(namespace: &str) -> Api<Version> {
    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = Client::from(kubeconfig);

    let versions: Api<Version> = Api::namespaced(client, namespace);

    return versions;
}

pub async fn get_versions(namespace: &str) -> anyhow::Result<Vec<Version>> {
    let versions = get_api(namespace);

    let lp = ListParams::default();
    let res = versions.list(&lp).await?;

    let items = res.items.into_iter().collect::<Vec<Version>>();

    Ok(items)
}

pub async fn add_version(namespace: &str, resource: &str, pipeline: &str, version: &str) -> anyhow::Result<()> {
    let versions = get_api(namespace);

    let pp = PostParams::default();
    let body = get_version_body(resource, pipeline, version)?;

    versions.create(&pp, &body).await?;

    Ok(())
}
