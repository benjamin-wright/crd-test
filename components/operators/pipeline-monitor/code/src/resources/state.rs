#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SecretKey {
    pub key: String,
    pub path: String
}
impl PartialEq for SecretKey {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.path == other.path
    }
}

impl Eq for SecretKey {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Secret {
    pub name: String,
    #[serde(rename(serialize = "mountPath", deserialize = "mountPath"))]
    pub mount_path: String,
    pub keys: Vec<SecretKey>
}

impl PartialEq for Secret {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.mount_path == other.mount_path && self.keys == other.keys
    }
}

impl Eq for Secret {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnvVar {
    pub name: String,
    pub value: String
}

impl PartialEq for EnvVar {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}

impl Eq for EnvVar {}

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug)]
#[kube(group = "minion.ponglehub.com", version = "v1", namespaced)]
pub struct ResourceSpec {
    pub image: String,
    pub secrets: Vec<Secret>,
    pub env: Vec<EnvVar>
}