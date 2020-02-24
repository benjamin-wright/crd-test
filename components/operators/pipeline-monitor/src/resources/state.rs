use kube::api::{ Object, Void };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Resource {
    pub image: String,
    #[serde(rename(serialize = "additionalVars", deserialize = "additionalVars"))]
    pub additional_vars: Vec<String>
}

pub type KubeResource = Object<Resource, Void>;