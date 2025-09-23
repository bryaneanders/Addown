use crate::models::{Game, GameResponse, GameArrayResponse, Mod, ModArrayResponse, ModResponse};
use crate::config::CurseForgeConfig;

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
    mod_id: i32,
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