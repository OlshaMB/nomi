use crate::{
    bootstrap::{profile::Loader, ClientAuth, ClientBootstrap, ClientSettings, ClientVersion},
    configs::launcher::Launcher,
    downloads::{
        launcher_manifest::{LauncherManifest, LauncherManifestVersion},
        Download,
    },
    utils::GetPath,
};

use anyhow::Result;
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize, Clone)]
struct Downloading {
    state: bool,
}

pub async fn download_version(_id: String) -> Result<(), ()> {
    let _load: Download = Download::new().await;

    // load.download(id, GetPath::game().to_str().unwrap().to_string())
    //   .await
    //   .unwrap();

    println!("1");
    tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
    println!("2");

    Ok(())
}

pub async fn get_manifest() -> Result<Vec<LauncherManifestVersion>> {
    let resp: LauncherManifest =
        reqwest::get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
            .await
            .map_err(|error| CommandsError::FailedToDownloadManifest(error))?
            .json()
            .await
            .map_err(|error| CommandsError::CantParseManifestToJson(error))?;

    Ok(resp.versions)
}

pub async fn get_config() -> Result<Launcher, ()> {
    let launcher_config = Launcher::from_file(None);

    Ok(launcher_config)
}

pub async fn launch(username: String, version: String, loader_verion: String) -> Result<()> {
    let bootstrap = ClientBootstrap::new(ClientSettings {
        assets: GetPath::game().join("assets"),
        auth: ClientAuth {
            username,
            access_token: None,
            uuid: Some(uuid::Uuid::new_v4().to_string()),
        },
        game_dir: GetPath::game(),
        java_bin: GetPath::java_bin().ok_or_else(|| CommandsError::CantFindJavaBin)?,
        libraries_dir: GetPath::game().join("libraries"),
        manifest_file: GetPath::game()
            .join("versions")
            .join(&loader_verion)
            .join(format!("{}.json", version)),
        natives_dir: GetPath::game()
            .join("versions")
            .join(&loader_verion)
            .join("natives"),
        version: ClientVersion {
            version: loader_verion.clone(),
            version_type: crate::bootstrap::VersionType::Release,
            loader: Loader::Quilt,
        },
        version_jar_file: GetPath::game()
            .join("versions")
            .join(&loader_verion)
            .join(format!("{}.jar", loader_verion)),
        // profile_path: Some(
        //     GetPath::versions()
        //         .join(&loader_verion)
        //         .join(format!("{}.json", loader_verion)),
        // ),
        profile_path: None,
    });

    bootstrap.launch()?;

    Ok(())
}

#[derive(Error, Debug)]
pub enum CommandsError {
    #[error("Can't find java executables")]
    CantFindJavaBin,

    #[error("Failed to download minecraft manifest file")]
    FailedToDownloadManifest(reqwest::Error),

    #[error("Can't parse minecraft manifest file to json")]
    CantParseManifestToJson(reqwest::Error),
}
