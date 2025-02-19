use color_eyre::Result;
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Flex, Layout, Rect},
    style::{Modifier, Style, palette::tailwind::*},
    text::Line,
    widgets::{Block, Borders, HighlightSpacing, List, ListState, StatefulWidget, Widget},
};

const MENU_HEADER_STYLE: Style = Style::new()
    .fg(ZINC.c100)
    .bg(BLUE.c600)
    .add_modifier(Modifier::BOLD);
const HEADER_STYLE: Style = Style::new().fg(ROSE.c500).add_modifier(Modifier::BOLD);
const SELECTED_STYLE: Style = Style::new().bg(ZINC.c700).add_modifier(Modifier::BOLD);

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}

struct App {
    should_exit: bool,
    menu: MenuList,
}

struct MenuList {
    items: Vec<&'static str>,
    state: ListState,
}

impl FromIterator<&'static str> for MenuList {
    fn from_iter<I: IntoIterator<Item = &'static str>>(iter: I) -> Self {
        let items = iter.into_iter().collect();
        let state = ListState::default();
        Self { items, state }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_exit: false,
            menu: MenuList::from_iter(["40L", "Blitz", "txLadder"]),
        }
    }
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('c') => {
                if key.modifiers.contains(event::KeyModifiers::CONTROL) {
                    self.should_exit = true;
                }
            }
            _ => {}
        }
    }

    fn select_next(&mut self) {
        self.menu.state.select_next();
    }
    fn select_previous(&mut self) {
        self.menu.state.select_previous();
    }
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area] =
            Layout::vertical([Constraint::Length(2), Constraint::Fill(1)]).areas(area);

        let list_area = center(main_area, Constraint::Length(30), Constraint::Length(5));

        App::render_header(header_area, buf);
        self.render_list(list_area, buf);
    }
}

impl App {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Block::new()
            .title(Line::raw("  txtris  ").centered().style(HEADER_STYLE))
            .borders(Borders::TOP)
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw(" Menu ").centered().style(MENU_HEADER_STYLE))
            .borders(Borders::ALL);

        let items: Vec<&'static str> = self.menu.items.clone();

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(" ")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.menu.state);
    }
}
