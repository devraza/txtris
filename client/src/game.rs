use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

pub fn render(area: Rect, buf: &mut Buffer) {
    let block = Block::new()
        .padding(Padding::symmetric(2, 1))
        .borders(Borders::ALL);

    Paragraph::new("").block(block).render(area, buf);
}
