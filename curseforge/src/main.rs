use std::sync::{Arc, Mutex};
use std::time::Duration;
use curseforge::cli_loop::{crate_rustyline_background_loop, create_ctrlc_background_loop, main_loop, CtrlCState, InputEvent};

mod config;
mod curseforge_api;
mod installed_mods;
mod models;
mod mod_table;

#[tokio::main]
async fn main() {

    println!("Welcome to Kubellm Interactive CLI!");
    println!("Type 'help' for available commands or 'exit' to quit.");
    println!("Press Ctrl+C twice quickly to force exit.\n");

    let ctrl_c_state = Arc::new(Mutex::new(CtrlCState::default()));
    let ctrl_c_timeout = Duration::from_secs(2);

    // Channel for communication between rustyline and main async task
    let (input_tx, mut input_rx) = tokio::sync::mpsc::unbounded_channel::<InputEvent>();

    // Spawn rustyline in a blocking thread (always listening)
    let input_tx_clone = input_tx.clone();
    let rusty_ctrl_c_state_clone = ctrl_c_state.clone();
    crate_rustyline_background_loop(ctrl_c_timeout, input_tx_clone, rusty_ctrl_c_state_clone);

    // Background task to clear Ctrl+C timeout messages
    let ctrl_c_state_clone = ctrl_c_state.clone();
    create_ctrlc_background_loop(ctrl_c_timeout, ctrl_c_state_clone);

    // Main async loop - handles both commands and input
    main_loop(ctrl_c_state, &mut input_rx).await;

    /*let game = curseforge_api::get_game_info(1).await.unwrap();
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
    table.populate_mods_table(game_mods).unwrap();
    table.printstd();

    installed_mods::get_installed_mods().await.unwrap();*/
}