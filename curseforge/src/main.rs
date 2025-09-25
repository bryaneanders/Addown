mod config;
mod curseforge_api;
mod installed_mods;
mod models;
mod mod_table;

#[tokio::main]
async fn main() {
let game = curseforge_api::get_game_info(1).await.unwrap();
    println!("Game: {} (ID: {}, Slug: {})", game.name, game.id, game.slug);

    let games = curseforge_api::get_games_info().await.unwrap();
    println!("\nGames ({} total):", games.len());
    for game in &games {
        println!(
            "  - {} (ID: {}, Status: {})",
            game.name, game.id, game.status
        );
    }

    let game_mod = curseforge_api::get_mod_info(1232053).await.unwrap();
    //println!("\n Mod: {:?}", game_mod);
    println!("\nMod: {} ({}), by {}", game_mod.name, game_mod.id, game_mod.authors[0].name);
    println!("\nMod URL: {:?}", game_mod.latest_files[0].download_url);

    // Download the file
    if let Some(download_url) = &game_mod.latest_files[0].download_url {
        println!("Downloading file from: {}", download_url);

        let response = reqwest::get(download_url).await.unwrap();
        let filename = &game_mod.latest_files[0].file_name;

        let bytes = response.bytes().await.unwrap();
        std::fs::write(filename, bytes).unwrap();

        println!(
            "Downloaded {} ({} bytes)",
            filename,
            std::fs::metadata(filename).unwrap().len()
        );
    } else {
        println!("No download URL available for this file");
    }

    let game_mods = curseforge_api::search_mods(1, "cell").await.unwrap();
    println!("\nSearch Results ({} total):", game_mods.len());
    for game_mod in &game_mods {
        println!("  - {} (ID: {}). About: {}", game_mod.name, game_mod.id, game_mod.summary);
    }

    let mut table = mod_table::ModTable::new();
    table.populate_mod_table(game_mods).unwrap();
    table.printstd();

    installed_mods::get_installed_mods().await.unwrap();
}