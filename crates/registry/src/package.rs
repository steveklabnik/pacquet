use std::collections::HashMap;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::RegistryError;

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageDistribution {
    pub integrity: String,
    #[serde(alias = "npm-signature")]
    pub npm_signature: Option<String>,
    pub shasum: String,
    pub tarball: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageVersion {
    #[serde(alias = "_npmVersion")]
    pub npm_version: String,
    #[serde(alias = "_nodeVersion")]
    pub node_version: Option<String>,
    pub dist: PackageDistribution,
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(alias = "devDependencies")]
    pub dev_dependencies: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub name: String,
    #[serde(alias = "dist-tags")]
    dist_tags: HashMap<String, String>,
    versions: HashMap<String, PackageVersion>,
}

impl Package {
    pub async fn from_registry(client: &Client, package_url: &str) -> Package {
        client
            .get(package_url)
            .header("user-agent", "pacquet-cli")
            .header("content-type", "application/json")
            .send()
            .await
            .or(Err(RegistryError::Network(package_url.to_string())))
            .unwrap()
            .json::<Package>()
            .await
            .or(Err(RegistryError::Serialization(package_url.to_string())))
            .unwrap()
    }

    pub fn get_latest_tag(&self) -> &String {
        self.dist_tags
            .get("latest")
            .ok_or(RegistryError::MissingLatestTag(self.name.to_owned()))
            .unwrap()
    }

    pub fn get_latest_version(&self) -> &PackageVersion {
        self.versions
            .get(self.get_latest_tag())
            .ok_or(RegistryError::MissingVersionRelease(
                self.get_latest_tag().to_owned(),
                self.name.to_owned(),
            ))
            .unwrap()
    }

    pub fn get_tarball_url(&self) -> &str {
        self.get_latest_version().to_owned().dist.tarball.as_str()
    }
}
