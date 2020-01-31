#[macro_use]
extern crate serde_derive;

use kube::{
    api::{Object, RawApi, Informer, WatchEvent, Void},
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

fn main() {
    let kubeconfig = config::load_kube_config().expect("kubeconfig failed to load");
    let client = APIClient::new(kubeconfig);
    let namespace = "default";
    let resource = RawApi::customResource("tasks")
        .group("minion.ponglehub.com")
        .within(&namespace);

    let informer = Informer::raw(client, resource).init().expect("informer init failed");
    loop {
        informer.poll().expect("informer poll failed");

        // Now we just do something each time a new book event is triggered.
        while let Some(event) = informer.pop() {
            handle(event);
        }
    }
}

fn handle(event: WatchEvent<KubeTask>) {
    println!("Something happened to a task")
}
