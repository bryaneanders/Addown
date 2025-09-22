mod config;
mod curseforge_api;
mod models;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let game = curseforge_api::get_game_info(1).await.unwrap();
    println!("Game: {} (ID: {}, Slug: {})", game.name, game.id, game.slug);

    let games = curseforge_api::get_games_info().await.unwrap();
    println!("\nGames ({} total):", games.len());
    for game in &games {
        println!("  - {} (ID: {}, Status: {})", game.name, game.id, game.status);
    }

    let game_mod = curseforge_api::get_mod_info(1232053).await.unwrap();
    println!("\n Mod: {:?}", game_mod);
    println!("\nMod URL: {:?}", game_mod.latest_files[0].download_url);

    // Download the file
    let download_url = &game_mod.latest_files[0].download_url;
    println!("Downloading file from: {}", download_url);

    let response = reqwest::get(download_url).await.unwrap();
    let filename = &game_mod.latest_files[0].file_name;

    let bytes = response.bytes().await.unwrap();
    std::fs::write(filename, bytes).unwrap();

    println!("Downloaded {} ({} bytes)", filename, std::fs::metadata(filename).unwrap().len());
}
