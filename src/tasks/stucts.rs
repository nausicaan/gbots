use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub sync_token: String,
    pub creation_time: String,
    pub prefixes: Vec<Prefix>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prefix {
    #[serde(rename = "ipv4Prefix")]
    pub ipv4prefix: String,
}
