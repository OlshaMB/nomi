use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type Meta = Vec<VersionLoader>;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VersionLoader {
    pub loader: Loader,
    pub intermediary: Intermediary,
    pub launcher_meta: LauncherMeta,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Loader {
    pub separator: String,
    pub build: i32,
    pub maven: String,
    pub version: String,
    pub stable: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Intermediary {
    pub maven: String,
    pub version: String,
    pub stable: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LauncherMeta {
    pub version: i32,
    pub libraries: Libraries,
    pub main_class: Value,
    pub arguments: Option<Arguments>,
    pub launchwrapper: Option<Launchwrapper>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Libraries {
    pub client: Vec<Library>,
    pub common: Vec<Library>,
    pub server: Vec<Server>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Library {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    #[serde(rename = "_comment")]
    pub comment: String,
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MainClass {
    pub client: String,
    pub server: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arguments {
    pub client: Vec<Value>,
    pub common: Vec<Value>,
    pub server: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Launchwrapper {
    pub tweakers: Tweakers,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tweakers {
    pub client: Vec<String>,
    pub common: Vec<Value>,
    pub server: Vec<String>,
}

/// Profile
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FabricProfile {
    pub id: String,
    pub inherits_from: String,
    pub release_time: String,
    pub time: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub main_class: String,
    pub arguments: FabricProfileArguments,
    pub libraries: Vec<Library>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FabricProfileArguments {
    pub game: Value,
    pub jvm: Vec<String>,
}