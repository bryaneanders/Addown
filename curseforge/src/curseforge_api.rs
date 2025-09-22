use crate::models::{Game, GameResponse};
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

    println!("{:#?}", game);
    Ok(game)
}