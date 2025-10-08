pub mod display;
pub mod pokemon;

use crate::display::*;
use iced::Font;

fn main() -> anyhow::Result<()> {
    iced::application(State::with_pool, update, view)
        .default_font(Font::MONOSPACE)
        .theme(theme)
        .run()?;
    Ok(())
}
