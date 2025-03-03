// Main menu sections
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, KeyCode, KeyEvent},
    layout::{Constraint, Layout, Rect},
    style::{
        Modifier, Style, Stylize,
        palette::tailwind::*,
    },
    text::{Line, Span},
    widgets::{
        Block, Borders, HighlightSpacing, List, ListState, Padding, Paragraph, StatefulWidget,
        Widget,
    },
};

use crate::tui;

const GAME_HEADER_STYLE: Style = Style::new()
    .fg(ZINC.c100)
    .bg(BLUE.c600)
    .add_modifier(Modifier::BOLD);
const PROFILE_HEADER_STYLE: Style = Style::new()
    .fg(ZINC.c100)
    .bg(VIOLET.c600)
    .add_modifier(Modifier::BOLD);

const HEADER_STYLE: Style = Style::new().fg(ROSE.c500).add_modifier(Modifier::BOLD);
const SELECTED_STYLE: Style = Style::new().bg(NEUTRAL.c700).add_modifier(Modifier::BOLD);

const FOOTER_LEFT_STYLE: Style = Style::new().fg(PURPLE.c400).add_modifier(Modifier::BOLD);
const FOOTER_RIGHT_STYLE: Style = Style::new().fg(LIME.c400);

pub fn render_header(area: Rect, buf: &mut Buffer) {
    Block::new()
        .title(Line::raw("  txtris  ").centered().style(HEADER_STYLE))
        .borders(Borders::TOP)
        .render(area, buf);
}

pub fn render_footer(area: Rect, buf: &mut Buffer) {
    Block::new()
        .title(
            Line::raw(" Atiran © 2025 ")
                .left_aligned()
                .style(FOOTER_LEFT_STYLE),
        )
        .title(
            Line::raw(format!("v{} ", env!("CARGO_PKG_VERSION")))
                .right_aligned()
                .style(FOOTER_RIGHT_STYLE),
        )
        .borders(Borders::BOTTOM)
        .render(area, buf);
}

pub fn render_profile(area: Rect, buf: &mut Buffer) {
    let block = Block::new()
        .title(
            Line::raw(" Profile ")
                .centered()
                .style(PROFILE_HEADER_STYLE),
        )
        .padding(Padding::symmetric(3, 1))
        .borders(Borders::ALL);

    let text = vec![
        Line::from(Span::styled("<Username>", Style::new().bold().underlined())),
        Line::from(vec![
            Span::styled("40L: ", Style::new().blue()),
            Span::raw("N/A"),
        ]),
        Line::from(vec![
            Span::styled("Blitz: ", Style::new().green()),
            Span::raw("N/A"),
        ]),
        Line::from(vec![
            Span::styled("txLadder: ", Style::new().magenta()),
            Span::raw("N/A"),
        ]),
    ];

    Paragraph::new(text).block(block).render(area, buf);
}

#[derive(Clone)]
pub struct OptionList {
    items: Vec<&'static str>,
    state: ListState,
}

impl FromIterator<&'static str> for OptionList {
    fn from_iter<I: IntoIterator<Item = &'static str>>(iter: I) -> Self {
        let items = iter.into_iter().collect();
        let state = ListState::default();
        Self { items, state }
    }
}

impl OptionList {
    pub fn handle_key(mut self, key: KeyEvent) -> tui::Mode {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => tui::Mode::Exit,
            KeyCode::Char('j') | KeyCode::Down => {
                self.select_next();
                tui::Mode::MainMenu(self)
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.select_previous();
                tui::Mode::MainMenu(self)
            }
            KeyCode::Char('c') => {
                if key.modifiers.contains(event::KeyModifiers::CONTROL) {
                    tui::Mode::Exit
                } else {
                    tui::Mode::MainMenu(self)
                }
            }
            KeyCode::Enter => match self.items.get(self.state.selected().unwrap_or_default()) {
                Some(&"Blitz") => tui::Mode::Blitz,
                Some(&"40L") => tui::Mode::FortyL,
                Some(&"txLadder") => tui::Mode::TxLadder,
                _ => tui::Mode::Exit,
            },
            _ => tui::Mode::MainMenu(self),
        }
    }

    fn select_next(&mut self) {
        self.state.select_next();
    }

    fn select_previous(&mut self) {
        self.state.select_previous();
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw(" Game ").centered().style(GAME_HEADER_STYLE))
            .padding(Padding::symmetric(2, 1))
            .borders(Borders::ALL);

        let items: Vec<&'static str> = self.items.clone();

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(" ")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}

pub fn render_menu(area: Rect, menu_list: &mut OptionList, buf: &mut Buffer) {
    let [profile_area, list_outer_area] =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(area);

    render_profile(profile_area, buf);

    menu_list.render(list_outer_area, buf);
}
