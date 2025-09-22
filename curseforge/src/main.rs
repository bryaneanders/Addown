mod config;
mod curseforge_api;
mod models;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let game = curseforge_api::get_game_info(1).await.unwrap();
    println!("Game Info: {:?}", game);
}
