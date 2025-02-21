use color_eyre::Result;

mod game;
mod menu;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = game::Mode::default().run(terminal);
    ratatui::restore();
    app_result
}
