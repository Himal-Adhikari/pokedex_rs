pub mod display;
pub mod pokemon;

use crate::display::*;
use iced::Font;
use sqlx::sqlite::SqlitePool;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect("pokedex.sqlite").await?;
    // let name = String::from("pik");

    // let _pokemon = get_pokemons(name, &pool).await;
    Ok(iced::application("Pokedex", update, view)
        .default_font(Font::MONOSPACE)
        .run_with(|| State::with_pool(pool))?)
}
