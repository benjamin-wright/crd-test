#[macro_use]
extern crate serde_derive;

use futures::StreamExt;
use kube::{
    api::{Informer, Object, RawApi, Void, WatchEvent},
    client::APIClient,
    config,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub pipeline: String,
    pub image: String,
    pub inputs: Option<Vec<String>>,
}

type KubeTask = Object<Task, Void>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = APIClient::new(kubeconfig);

    // Set a namespace. We're just hard-coding for now.
    let namespace = "default";

    let resource = RawApi::customResource("tasks")
        .group("minion.ponglehub.com");

    // Create our informer and start listening.
    let informer = Informer::raw(client, resource)
        .init()
        .await?;

    loop {
        let mut tasks = informer.poll().await?.boxed();

        // Now we just do something each time a new book event is triggered.
        while let Some(event) = tasks.next().await {
            handle(event?);
        }
    }
}

fn handle(event: WatchEvent<KubeTask>) {
    match event {
        WatchEvent::Added(task) => println!(
            "Added a task {} from pipeline '{}'",
            task.metadata.name, task.spec.pipeline
        ),
        WatchEvent::Deleted(task) => println!("Deleted a task {}", task.metadata.name),
        _ => println!("another event"),
    }
}
