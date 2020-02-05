#[macro_use]
extern crate serde_derive;

use futures::StreamExt;
use kube::{
    api::{Informer, ListParams, Object, RawApi, Void, WatchEvent},
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

    let tasks = RawApi::customResource("tasks")
        .group("minion.ponglehub.com");

    for t in tasks.list(&ListParams::default()).unwrap() {
        loadTask(t);
    }

    // Create our informer and start listening.
    let informer = Informer::raw(client, tasks)
        .init()
        .await?;

    loop {
        let mut taskEvents = informer.poll().await?.boxed();

        // Now we just do something each time a new task event is triggered.
        while let Some(event) = taskEvents.next().await {
            handle(event?);
        }
    }
}

fn handle(event: WatchEvent<KubeTask>) {
    match event {
        WatchEvent::Added(task) => loadTask(task),
        WatchEvent::Deleted(task) => removedTask(task),
        _ => println!("another event"),
    }
}

fn loadTask(task: KubeTask) {
    println!(
        "Added a task {}:{} from pipeline '{}'",
        task.metadata.namespace.expect("Namespace not defined"),
        task.metadata.name,
        task.spec.pipeline
    )
}

fn removedTask(task: KubeTask) {
    println!("Deleted a task {}", task.metadata.name)
}