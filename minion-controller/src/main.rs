#[macro_use]
extern create serde_derive;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub pipeline: String,
    pub image: String,
    pub inputs: Option<Vec<String>>,
}

fn main() {
    println!("Hello, world!");
}
