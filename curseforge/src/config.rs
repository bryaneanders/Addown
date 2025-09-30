use anyhow::{Context, Result};
use std::env;
use std::sync::OnceLock;
use dotenvy::dotenv;

#[derive(Debug)]
pub struct CurseForgeConfig {
    pub api_key: String,
    pub wow_path: String,
    pub path_suffix: String,
}

static CONFIG: OnceLock<CurseForgeConfig> = OnceLock::new();

impl CurseForgeConfig {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();
        let api_key = env::var("CURSEFORGE_API_KEY")
            .context("CURSEFORGE_API_KEY must be set in the environment")?;

        Ok(CurseForgeConfig {
            api_key: api_key,
            wow_path: "/home/dev/world_of_warcraft".to_string(),
            path_suffix: "/_retail_/Interface/AddOns".to_string()
        })
    }

    pub fn addons_path(&self) -> String {
        format!("{}{}", self.wow_path, self.path_suffix)
    }

    pub fn get() -> &'static CurseForgeConfig {
        CONFIG.get_or_init(|| Self::from_env().expect("Failed to load configuration"))
    }
}
