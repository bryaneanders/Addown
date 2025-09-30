use std::fs::File;
use std::io;
use crate::models::{Game, GameResponse, GameArrayResponse, Mod, ModArrayResponse, ModResponse};
use crate::config::CurseForgeConfig;
use zip::ZipArchive;

pub async fn get_game_info(
    game_id: i32,
) -> Result<Game, Box<dyn std::error::Error>> {
    let config = CurseForgeConfig::get();
    let client = reqwest::Client::new();
    let url = format!("https://api.curseforge.com/v1/games/{}", game_id);

    let response = client
        .get(&url)
        .header("x-api-key", &config.api_key)
        .header("accept", "application/json")
        .send()
        .await?;

    let response_text = response.text().await?;
    let game_response: GameResponse = serde_json::from_str(&response_text)?;
    let game = game_response.data;

    Ok(game)
}

pub async fn get_games_info() -> Result<Vec<Game>, Box<dyn std::error::Error>> {
    let config = CurseForgeConfig::get();
    let client = reqwest::Client::new();
    let url = format!("https://api.curseforge.com/v1/games");

    let response = client
        .get(&url)
        .header("x-api-key", &config.api_key)
        .header("accept", "application/json")
        .send()
        .await?;

    let response_text = response.text().await?;
    let games_response: GameArrayResponse = serde_json::from_str(&response_text)?;
    let games = games_response.data;

    Ok(games)
}

pub async fn get_mod_info(
    mod_id: u32,
) -> Result<Mod, Box<dyn std::error::Error>> {
    let config = CurseForgeConfig::get();
    let client = reqwest::Client::new();
    let url = format!("https://api.curseforge.com/v1/mods/{}", mod_id);

    let response = client
        .get(&url)
        .header("x-api-key", &config.api_key)
        .header("accept", "application/json")
        .send()
        .await?;

    let response_text = response.text().await?;
    let mod_response: ModResponse = serde_json::from_str(&response_text)?;
    let mod_info = mod_response.data;
    Ok(mod_info)
}

pub async fn get_mod_file(
    file_id: u32,
    filename: &str,
) -> Result<String, Box<dyn std::error::Error>>
{
    let config = CurseForgeConfig::get();
    let file_id_str = file_id.to_string();
    let file_path = &(config.addons_path() + "/" + filename);
    let prefix = &file_id_str[0..4];
    let suffix = &file_id_str[4..];
    let encoded_filename = urlencoding::encode(filename);

    let download_url = format!(
        "https://edge.forgecdn.net/files/{}/{}/{}",
        prefix, suffix, encoded_filename
    );

    let response = reqwest::get(download_url).await.unwrap();
    let bytes = response.bytes().await.unwrap();
    std::fs::write(file_path, bytes).unwrap();
    //println!("Top level dirs: {}", get_top_level_dirs(file_path)?.join(", "));
    println!(
        "Downloaded {} ({} bytes)",
        file_path,
        std::fs::metadata(file_path).unwrap().len()
    );

    for dir in get_top_level_dirs(file_path)? {
        let dir_path = format!("{}/{}", config.addons_path(), dir);
        std::fs::remove_dir_all(&dir_path).ok();
    }

    unzip_file(file_path, &config.addons_path()).expect("TODO: panic message");
    std::fs::remove_file(file_path).ok();

    Ok("ok".to_string()) // todo fix
}

pub fn unzip_file(zip_path: &str, extract_to: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = std::path::Path::new(extract_to).join(file.name());

        if file.is_dir() {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

pub fn get_top_level_dirs(zip_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    let mut dirs = std::collections::HashSet::new();
    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        let path = std::path::Path::new(file.name());
        if let Some(first_component) = path.components().next() {
            if let Some(name) = first_component.as_os_str().to_str() {
                dirs.insert(name.to_string());
            }
        }
    }

    Ok(dirs.into_iter().collect())
}

pub async fn search_mods(
    game_id: i32,
    search_filter: &str,
) -> Result<Vec<Mod>, Box<dyn std::error::Error>> {
    let config = CurseForgeConfig::get();
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.curseforge.com/v1/mods/search?gameId={}&searchFilter={}",
        game_id, search_filter
    );

    let response = client
        .get(&url)
        .header("x-api-key", &config.api_key)
        .header("accept", "application/json")
        .send()
        .await?;

    let response_text = response.text().await?;
    //println!("Search API Response: {}", response_text);
    let mods_response: ModArrayResponse = serde_json::from_str(&response_text)?;
    let mods = mods_response.data;

    Ok(mods)
}