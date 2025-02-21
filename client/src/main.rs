use color_eyre::Result;

mod tui;
mod menu;
mod util;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = tui::Mode::default().run(terminal);
    ratatui::restore();
    app_result
}
