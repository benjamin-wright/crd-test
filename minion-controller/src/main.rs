#[macro_use]
extern crate serde_derive;

use futures::StreamExt;
use kube::{
    api::{Informer, Object, RawApi, Void, WatchEvent, Reflector},
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
    prepare_state().await?;
    listen_for_changes().await?;

    Ok(())
}

fn get_tasks() -> RawApi {
    return RawApi::customResource("tasks")
        .group("minion.ponglehub.com");
}

async fn prepare_state() -> Result<(), anyhow::Error> {
    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = APIClient::new(kubeconfig);

    let tasks_api = get_tasks();

    let task_reflector: Reflector<KubeTask> = Reflector::raw(client, tasks_api)
        .timeout(10)
        .init()
        .await?;

    task_reflector.poll().await?;

    task_reflector.state().await.into_iter().for_each(|tasks| {
        for task in &tasks {
            println!(
                "Found Task {}:{} from pipeline '{}'",
                task.metadata.namespace.as_ref().expect("Namespace not defined"),
                task.metadata.name,
                task.spec.pipeline
            );
        }
    });

    return Ok(());
}

async fn listen_for_changes() -> anyhow::Result<()> {
    // Load the kubeconfig file.
    let kubeconfig = config::incluster_config().expect("Failed to load kube config");

    // Create a new client
    let client = APIClient::new(kubeconfig);

    let tasks_api = get_tasks();

    // Create our informer and start listening.
    let informer = Informer::raw(client, tasks_api)
        .init()
        .await?;

    loop {
        let mut task_events = informer.poll().await?.boxed();

        // Now we just do something each time a new task event is triggered.
        while let Some(event) = task_events.next().await {
            handle(event?);
        }
    }
}

fn handle(event: WatchEvent<KubeTask>) {
    match event {
        WatchEvent::Added(task) => load_task(task),
        WatchEvent::Deleted(task) => removed_task(task),
        _ => println!("another event"),
    }
}

fn load_task(task: KubeTask) {
    println!(
        "Added a task {}:{} from pipeline '{}'",
        task.metadata.namespace.as_ref().expect("Namespace not defined"),
        task.metadata.name,
        task.spec.pipeline
    );
}

fn removed_task(task: KubeTask) {
    println!("Deleted a task {}", task.metadata.name);
}
