use ratatui_core::layout::Position;
use ratatui_core::terminal::Frame;
use ratatui_widgets::block::Block;
use ratatui_widgets::borders::Borders;
use ratatui_widgets::paragraph::Paragraph;

use crate::app::App;
use crate::ui;

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    if let Some(ref editor) = app.editor {
        let slug_label = format!(" {} ", editor.slug);
        let dirty_mark = if editor.dirty { " ●" } else { "" };
        let title_text = format!("{}{}", slug_label, dirty_mark);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(title_text)
            .title_bottom(format!(
                " Ln {} Col {} ",
                editor.buffer.cursor_y + 1,
                editor.buffer.cursor_x + 1
            ));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let (scroll_offset, end) =
            editor.buffer.visible_line_range(inner.height.saturating_sub(1) as usize);

        let mut text = String::new();
        for i in scroll_offset..end.min(editor.buffer.lines.len()) {
            let line_num = i + 1;
            let is_cursor_line = i == editor.buffer.cursor_y;
            let num_prefix = if is_cursor_line { ">" } else { " " };
            let line_content = &editor.buffer.lines[i];
            text.push_str(&format!("{}{:>3} {}\n", num_prefix, line_num, line_content));
        }

        let paragraph = Paragraph::new(text);
        frame.render_widget(paragraph, inner);

        let cursor_x = ui::editor_visible_cursor_x(editor);
        let cursor_y = editor.buffer.cursor_y.saturating_sub(scroll_offset);
        frame.set_cursor_position(Position::new(
            inner.x + cursor_x as u16 + 5,
            inner.y + cursor_y as u16,
        ));
    }
}
