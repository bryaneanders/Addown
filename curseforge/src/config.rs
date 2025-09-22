use anyhow::{Context, Result};
use std::env;
use std::sync::OnceLock;
use dotenvy::dotenv;

#[derive(Debug)]
pub struct CurseForgeConfig {
    pub api_key: String,
}

static CONFIG: OnceLock<CurseForgeConfig> = OnceLock::new();

impl CurseForgeConfig {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();
        let api_key = env::var("CURSEFORGE_API_KEY")
            .context("CURSEFORGE_API_KEY must be set in the environment")?;

        Ok(CurseForgeConfig { api_key })
    }

    pub fn get() -> &'static CurseForgeConfig {
        CONFIG.get_or_init(|| Self::from_env().expect("Failed to load configuration"))
    }
}
