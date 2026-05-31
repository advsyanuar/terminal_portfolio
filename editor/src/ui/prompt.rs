use ratatui_core::layout::{Position, Rect};
use ratatui_core::terminal::Frame;
use ratatui_widgets::block::Block;
use ratatui_widgets::borders::Borders;
use ratatui_widgets::clear::Clear;
use ratatui_widgets::paragraph::Paragraph;

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let prompt = match app.prompt {
        Some(ref p) => p,
        None => return,
    };

    let popup_width = 50.min(area.width.saturating_sub(4));
    let popup_height = 5;
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;

    let popup_area = Rect {
        x: popup_x,
        y: popup_y,
        width: popup_width,
        height: popup_height,
    };

    frame.render_widget(Clear, popup_area);
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" INPUT ");
    frame.render_widget(block, popup_area);

    let input_text = format!(" {} {}", prompt.label, prompt.value);
    let paragraph = Paragraph::new(input_text.clone());
    let input_area = Rect {
        x: popup_area.x + 1,
        y: popup_area.y + 2,
        width: popup_area.width.saturating_sub(2),
        height: 1,
    };
    frame.render_widget(paragraph, input_area);

    let cursor_visible = input_text.len() < popup_area.width.saturating_sub(2) as usize;
    if cursor_visible {
        let offset = 1 + prompt.label.len() as u16 + prompt.cursor as u16 + 1;
        let _ = frame.set_cursor_position(Position::new(input_area.x + offset, input_area.y));
    }

    if let Some(ref err) = prompt.error {
        let err_area = Rect {
            x: popup_area.x + 1,
            y: popup_area.y + 3,
            width: popup_area.width.saturating_sub(2),
            height: 1,
        };
        let err_paragraph = Paragraph::new(err.clone());
        frame.render_widget(err_paragraph, err_area);
    }

    let hint_area = Rect {
        x: popup_area.x + 1,
        y: popup_area.y + popup_height - 1,
        width: popup_area.width.saturating_sub(2),
        height: 1,
    };
    frame.render_widget(Paragraph::new("Enter confirm · Esc cancel"), hint_area);
}
