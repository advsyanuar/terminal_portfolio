#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Enter,
    Esc,
    Tab,
    BackTab,
    Backspace,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    CtrlS,
    CtrlC,
    Null,
}

/// Convert ANSI escape sequences (from xterm.js onData) to Key.
pub fn convert_terminal_key(data: &str) -> Option<Key> {
    match data {
        "\r" => Some(Key::Enter),
        "\x7f" => Some(Key::Backspace),
        "\t" => Some(Key::Tab),
        "\x1b" => Some(Key::Esc),
        "\x1b[A" => Some(Key::Up),
        "\x1b[B" => Some(Key::Down),
        "\x1b[C" => Some(Key::Right),
        "\x1b[D" => Some(Key::Left),
        "\x1b[H" => Some(Key::Home),
        "\x1b[F" => Some(Key::End),
        "\x1b[5~" => Some(Key::PageUp),
        "\x1b[6~" => Some(Key::PageDown),
        "\x1b[3~" => Some(Key::Delete),
        "\x1b[Z" => Some(Key::BackTab),
        s if s.len() == 1 => s.chars().next().map(Key::Char),
        _ => None,
    }
}
