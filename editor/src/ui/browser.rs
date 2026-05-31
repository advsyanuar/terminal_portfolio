use ratatui_core::layout::Rect;
use ratatui_core::terminal::Frame;
use ratatui_widgets::block::Block;
use ratatui_widgets::borders::Borders;
use ratatui_widgets::paragraph::Paragraph;

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let title = format!(" {} ENTRIES ", app.content_type.label());

    let block = Block::default().borders(Borders::ALL).title(title);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut text = String::new();
    for (i, entry) in app.entries.iter().enumerate() {
        let prefix = if i == app.browser_cursor { " ▸ " } else { "   " };
        text.push_str(&format!("{}{}\n", prefix, entry.title));
    }
    text.push_str("  [ N ] NEW ENTRY\n");

    let list = Paragraph::new(text);
    frame.render_widget(list, inner);

    let hint = Paragraph::new("↑↓ navigate  ·  Enter open  ·  n new  ·  Esc back");
    let hint_area = Rect {
        x: area.x,
        y: area.y + area.height - 1,
        width: area.width,
        height: 1,
    };
    frame.render_widget(hint, hint_area);
}
