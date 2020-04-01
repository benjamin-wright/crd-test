#[derive(CustomResource, Deserialize, Serialize, Clone, Debug)]
#[kube(group = "minion.ponglehub.com", version = "v1", namespaced)]
pub struct ResourceSpec {
    pub image: String,
    #[serde(rename(serialize = "additionalVars", deserialize = "additionalVars"))]
    pub additional_vars: Vec<String>
}