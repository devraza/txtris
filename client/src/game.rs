use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style, palette::tailwind::*},
    symbols,
    text::Line,
    widgets::{Block, Borders, Widget},
};

use crate::tui;

const GAMEMODE_STYLE: Style = Style::new().bg(NEUTRAL.c700).add_modifier(Modifier::BOLD);
const TITLE_STYLE: Style = Style::new()
    .fg(NEUTRAL.c800)
    .bg(NEUTRAL.c100)
    .add_modifier(Modifier::BOLD);

pub fn render(mode: &mut tui::Mode, area: Rect, buf: &mut Buffer) {
    let gamemode = format!(" {} ", match mode {
        tui::Mode::Blitz => "Blitz",
        tui::Mode::FortyL => "40L",
        tui::Mode::TxLadder => "txLadder",
        _ => "",
    });

    let [mut hold_area, grid_area, mut next_area] =
        Layout::horizontal(Constraint::from_percentages([25, 50, 25])).areas(area);

    hold_area.height = 5;
    next_area.height = 16;

    Block::new()
        .title(Line::styled(" HOLD ", TITLE_STYLE).centered())
        .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
        .render(hold_area, buf);

    Block::new()
        .title(Line::styled(gamemode, GAMEMODE_STYLE).centered())
        .border_set(symbols::border::Set {
            top_left: symbols::line::NORMAL.horizontal_down,
            top_right: symbols::line::NORMAL.horizontal_down,
            ..symbols::border::PLAIN
        })
        .borders(Borders::ALL)
        .render(grid_area, buf);

    Block::new()
        .title(Line::styled(" NEXT ", TITLE_STYLE).centered())
        .borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM)
        .render(next_area, buf);
}
