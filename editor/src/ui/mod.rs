mod browser;
mod dashboard;
mod ed_panel;
mod prompt;

use ratatui_core::layout::Rect;
use ratatui_core::terminal::Frame;
use ratatui_widgets::paragraph::Paragraph;

use crate::app::{App, Screen};
use crate::editor::Editor;

pub fn render(frame: &mut Frame, app: &App) {
    match app.screen {
        Screen::Dashboard => dashboard::render(frame, app),
        Screen::Browser => browser::render(frame, app),
        Screen::Editor => ed_panel::render(frame, app),
        Screen::Prompt => {
            if app.editor.is_some() {
                ed_panel::render(frame, app);
            } else {
                browser::render(frame, app);
            }
            prompt::render(frame, app);
        }
    }

    if let Some(ref msg) = app.status_message {
        let msg_paragraph = Paragraph::new(msg.clone());
        let area = frame.area();
        let status_area = Rect {
            x: area.x,
            y: area.y + area.height - 1,
            width: area.width.min(msg.len() as u16 + 2),
            height: 1,
        };
        frame.render_widget(msg_paragraph, status_area);
    }
}

pub fn editor_visible_cursor_x(editor: &Editor) -> usize {
    let line = editor.buffer.current_line();
    let before = &line[..editor.buffer.cursor_x.min(line.len())];
    unicode_width::UnicodeWidthStr::width(before)
}
