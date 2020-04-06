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
pub struct EnvVar {
    pub name: String,
    pub value: String
}

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug)]
#[kube(group = "minion.ponglehub.com", version = "v1", namespaced)]
pub struct ResourceSpec {
    pub image: String,
    pub secrets: Vec<Secret>,
    pub env: Vec<EnvVar>
}