use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ClusterSize {
    Tiny,
    XSmall,
    Small,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Local,
    InCluster,
}
