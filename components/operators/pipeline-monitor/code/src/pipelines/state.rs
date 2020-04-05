#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Resource {
    pub name: String,
    pub trigger: bool
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

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug)]
#[kube(group = "minion.ponglehub.com", version = "v1", namespaced)]
pub struct PipelineSpec {
    pub resources: Vec<Resource>,
    pub steps: Vec<Step>
}