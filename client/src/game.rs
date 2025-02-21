use color_eyre::Result;
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

use crate::menu;
use crate::util::*;

#[derive(Clone)]
pub enum Mode {
    Exit,
    MainMenu(menu::OptionList),
    FortyL,
    Blitz,
    TxLadder,
    Config,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::MainMenu(menu::OptionList::from_iter(["40L", "Blitz", "txLadder", "Config"]))
    }
}

impl Mode {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            match self {
                Mode::Exit => {
                    break
                }
                _ => {
                    terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
                    if let Event::Key(key) = event::read()? {
                        self = self.handle_key(key);
                    };
                }
            }
        }
        Ok(())
    }

    fn handle_key(self, key: KeyEvent) -> Mode {
        if key.kind != KeyEventKind::Press {
            return self 
        }

        match self {
            Mode::MainMenu(menu_list) => menu_list.handle_key(key),
            _ => Mode::Exit,
        }
    }
}

impl Widget for &mut Mode {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(2),
        ])
        .areas(area);

        let center_area = center(
            main_area,
            Constraint::Length(50),
            Constraint::Percentage(60),
        );

        let [profile_area, list_outer_area] =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(center_area);

        menu::render_header(header_area, buf);

        match self {
            Mode::MainMenu(menu_list) => {
                menu::render_profile(profile_area, buf);
                menu_list.render(list_outer_area, buf);
            }
            _ => {},
        }
        menu::render_footer(footer_area, buf);
    }
}
