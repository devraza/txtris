#![warn(clippy::pedantic)]
#![allow(clippy::wildcard_imports)]

use color_eyre::Result;

mod game;
mod menu;
mod tui;
mod util;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = tui::Mode::default().run(terminal);
    ratatui::restore();
    app_result
}
