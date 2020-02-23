use std::collections::BTreeMap;
use kube::api::{ Object, Void };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SecretKey {
    pub key: String,
    pub path: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Secret {
    pub name: String,
    pub keys: Vec<SecretKey>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Resource {
    pub name: String,
    pub trigger: bool,
    pub secrets: Vec<Secret>,
    pub env: BTreeMap<String, String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Step {
    pub name: String,
    pub resource: Option<String>,
    pub action: Option<String>,
    pub path: Option<String>,
    pub image: Option<String>,
    pub command: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pipeline {
    pub resources: Vec<Resource>,
    pub steps: Vec<Step>
}

pub type KubePipeline = Object<Pipeline, Void>;