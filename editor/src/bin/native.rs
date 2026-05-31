use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui_core::terminal::Terminal;
use ratatui_crossterm::CrosstermBackend;

use editor::app::App;
use editor::key::Key;
use editor::storage::local::LocalStorage;
use editor::ui;

fn convert_event(ev: KeyEvent) -> Option<Key> {
    if ev.modifiers == KeyModifiers::CONTROL {
        return match ev.code {
            KeyCode::Char('s') => Some(Key::CtrlS),
            KeyCode::Char('c') => Some(Key::CtrlC),
            _ => None,
        };
    }
    match ev.code {
        KeyCode::Up => Some(Key::Up),
        KeyCode::Down => Some(Key::Down),
        KeyCode::Left => Some(Key::Left),
        KeyCode::Right => Some(Key::Right),
        KeyCode::Enter => Some(Key::Enter),
        KeyCode::Esc => Some(Key::Esc),
        KeyCode::Tab => Some(Key::Tab),
        KeyCode::BackTab => Some(Key::BackTab),
        KeyCode::Backspace => Some(Key::Backspace),
        KeyCode::Delete => Some(Key::Delete),
        KeyCode::Home => Some(Key::Home),
        KeyCode::End => Some(Key::End),
        KeyCode::PageUp => Some(Key::PageUp),
        KeyCode::PageDown => Some(Key::PageDown),
        KeyCode::Char(c) => Some(Key::Char(c)),
        _ => None,
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let mut app = App::new();
    let mut storage = LocalStorage::new(".");

    loop {
        terminal.draw(|frame| {
            ui::render(frame, &app);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                break;
            }
            if key.code == KeyCode::Char('q') {
                break;
            }
            if let Some(k) = convert_event(key) {
                app.handle_key(k, &mut storage);
            }
        }
    }

    disable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(LeaveAlternateScreen)?;
    Ok(())
}
