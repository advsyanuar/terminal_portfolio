use ratatui_core::layout::{Constraint, Direction, Layout, Rect};
use ratatui_core::style::{Color, Modifier, Style};
use ratatui_core::text::Text;
use ratatui_core::terminal::Frame;
use ratatui_widgets::block::Block;
use ratatui_widgets::borders::Borders;
use ratatui_widgets::paragraph::Paragraph;

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(25), Constraint::Min(1)])
        .split(main_layout[0]);

    render_left_panel(frame, app, content_layout[0]);
    render_right_panel(frame, app, content_layout[1]);
    render_footer(frame, app, main_layout[1]);
}

fn render_left_panel(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Menu ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .split(inner);

    let title = Paragraph::new(Text::styled(
        "MARKDOWN EDITOR v0.1",
        Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::BOLD),
    ));
    frame.render_widget(title, layout[0]);

    let blog_style = if app.content_type == crate::models::ContentType::Blog {
        Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::REVERSED)
    } else {
        Style::default().fg(Color::White)
    };
    let project_style = if app.content_type == crate::models::ContentType::Project {
        Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::REVERSED)
    } else {
        Style::default().fg(Color::White)
    };

    let blog_text = Paragraph::new(Text::styled("[ BLOG ]", blog_style));
    let project_text = Paragraph::new(Text::styled("[ PROJECTS ]", project_style));
    frame.render_widget(blog_text, layout[1]);
    frame.render_widget(project_text, layout[2]);
}

fn render_right_panel(frame: &mut Frame, app: &App, area: Rect) {
    let entries = match app.content_type {
        crate::models::ContentType::Blog => &app.project_entries,
        crate::models::ContentType::Project => &app.blog_entries,
    };

    let label = app.content_type.label();
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} ({}) ", label, entries.len()));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if entries.is_empty() {
        let empty = Paragraph::new(Text::styled(
            " No entries yet.\n Press 'n' to create one.",
            Style::default().fg(Color::DarkGray),
        ));
        frame.render_widget(empty, inner);
        return;
    }

    let max_rows = inner.height as usize;
    let title_col = inner.width.saturating_sub(14).max(10) as usize;
    let mut text = String::new();
    for (i, entry) in entries.iter().enumerate() {
        if i >= max_rows {
            break;
        }
        let title = if entry.title.len() > title_col {
            format!("{}…", &entry.title[..title_col])
        } else {
            entry.title.clone()
        };
        let date = if entry.date.len() > 10 {
            &entry.date[..10]
        } else {
            &entry.date
        };
        let pad = title_col.saturating_sub(title.len()) + 2;
        text.push_str(&format!(" {}{}│ {}\n", title, " ".repeat(pad), date));
    }

    let paragraph = Paragraph::new(Text::styled(text, Style::default().fg(Color::White)));
    frame.render_widget(paragraph, inner);
}

fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
    let text = format!(
        " ↑↓ select  ·  Enter browse  ·  n new  ·  Esc back  ·  q exit  ·  Ctrl+s save  ·│  {} ",
        app.content_type.label()
    );
    let paragraph = Paragraph::new(Text::styled(
        text,
        Style::default().fg(Color::DarkGray),
    ));
    frame.render_widget(paragraph, area);
}
