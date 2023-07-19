use log::{error, trace};
use anyhow::{Result, Context};
use reqwest::{blocking, Client};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tokio::task::spawn_blocking;

#[derive(Serialize, Deserialize, Debug)]
pub struct Assets {
    // pub objects: Vec<HashMap<String, AssetInformation>>
    pub objects: HashMap<String, AssetInformation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetInformation {
    pub hash: String,
    pub size: i64,
}

#[derive(Debug)]
pub struct AssetsDownload {
    assets: Assets,
    id: String,
    url: String,
}

impl AssetsDownload {
    pub async fn new(url: String, id: String) -> Result<Self> {
        Ok(
            Self {
                assets: Self::init(url.clone()).await?,
                id,
                url,
            }
        )   
    }

    async fn init(url: String) -> Result<Assets> {
        let data: Assets = Client::new()
            .get(url)
            .send()
            .await
            .context("failed to send get request")?
            .json()
            .await
            .context("failed to parse json")?;

        Ok(data)
    }

    // FIXME!!!:
    fn create_dir(&self, main_dir: &str, asset_dir_name: &str) -> Result<PathBuf> {
        let path = Path::new(main_dir)
            .join("assets")
            .join("objects")
            .join(asset_dir_name);

        let _ = std::fs::create_dir_all(&path);

        // TODO: remove this after debug
        trace!("Dir {} created successfully", path.to_string_lossy());

        return Ok(path);
    }

    pub async fn get_assets_json(&self, assets_dir: &String) -> Result<()> {
        let filen = format!("{}.json", self.id);
        let path = Path::new(&assets_dir)
            .join("assets")
            .join("indexes")
            .join(filen);

        let _ = std::fs::create_dir_all(path.parent().context("")?);

        let body = Client::new().get(&self.url).send().await?.text().await?;

        match std::fs::write(&path, body) {
            Ok(_) => trace!("Dowloaded successfully {}", path.to_string_lossy()),
            Err(e) => error!("Dowload error {}", e),
        }

        Ok(())
    }

    pub async fn download_assets(&self, dir: &str) -> Result<()>{
        for (_k, v) in self.assets.objects.iter() {
            let path = self.create_dir(dir, &v.hash[0..2])?;

            let mut file = std::fs::File::create(path.join(&v.hash)).unwrap();

            let url = format!(
                "https://resources.download.minecraft.net/{}/{}",
                &v.hash[0..2],
                v.hash
            );
            let _response =
                spawn_blocking(move || blocking::get(url).unwrap().copy_to(&mut file).unwrap())
                    .await;

            trace!(
                "Asset {:?} with hash {} dowloaded successfully",
                path.join(&v.hash),
                &v.hash[0..2]
            );
        }
        return Ok(());
    }
}
