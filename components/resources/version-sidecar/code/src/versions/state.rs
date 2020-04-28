use std::fmt;

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug)]
#[kube(group = "minion.ponglehub.com", version = "v1", namespaced)]
pub struct VersionSpec {
    pub resource: String,
    pub pipeline: String,
    pub version: String
}

impl fmt::Display for VersionSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {} ({})", self.resource, self.version, self.pipeline)
    }
}

impl PartialEq for VersionSpec {
    fn eq(&self, other: &Self) -> bool {
        self.resource == other.resource && self.pipeline == other.pipeline && self.version == other.version
    }
}

impl Eq for VersionSpec {}
