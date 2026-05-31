use ratatui_core::backend::{Backend, ClearType, WindowSize};
use ratatui_core::buffer::Cell;
use ratatui_core::layout::{Position, Size};
use ratatui_core::style::Color;

pub struct WasmBackend {
    cols: u16,
    rows: u16,
    output: String,
    cursor_pos: Position,
    cursor_visible: bool,
    grid: Vec<Vec<(char, Color)>>,
    initialized: bool,
}

impl WasmBackend {
    pub fn new(cols: u16, rows: u16) -> Self {
        let grid = vec![vec![(' ', Color::Reset); cols as usize]; rows as usize];
        Self {
            cols,
            rows,
            output: String::new(),
            cursor_pos: Position::new(0, 0),
            cursor_visible: false,
            grid,
            initialized: false,
        }
    }

    pub fn take_output(&mut self) -> String {
        std::mem::take(&mut self.output)
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;
        let mut new_grid = vec![vec![(' ', Color::Reset); cols as usize]; rows as usize];
        for y in 0..self.grid.len().min(rows as usize) {
            for x in 0..self.grid[y].len().min(cols as usize) {
                new_grid[y][x] = self.grid[y][x];
            }
        }
        self.grid = new_grid;
    }

    fn render_full_grid(&self) -> String {
        let mut out = String::new();
        out.push_str("\x1b[2J\x1b[H");
        for y in 0..self.rows as usize {
            if y > 0 {
                out.push_str("\r\n");
            }
            for x in 0..self.cols as usize {
                let (c, fg) = self.grid[y][x];
                let ansi = self.color_to_ansi(fg);
                out.push_str(ansi);
                out.push(c);
                out.push_str("\x1b[0m");
            }
        }
        out
    }

    fn push_cursor_ansi(&mut self) {
        if self.cursor_visible {
            self.output.push_str(&format!(
                "\x1b[{};{}H",
                self.cursor_pos.y + 1,
                self.cursor_pos.x + 1
            ));
        }
    }

    fn color_to_ansi(&self, color: Color) -> &'static str {
        match color {
            Color::Reset => "\x1b[39m",
            Color::Black => "\x1b[30m",
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
            Color::DarkGray => "\x1b[90m",
            Color::LightRed => "\x1b[91m",
            Color::LightGreen => "\x1b[92m",
            Color::LightYellow => "\x1b[93m",
            Color::LightBlue => "\x1b[94m",
            Color::LightMagenta => "\x1b[95m",
            Color::LightCyan => "\x1b[96m",
            _ => "\x1b[39m",
        }
    }
}

impl Backend for WasmBackend {
    type Error = std::io::Error;

    fn size(&self) -> Result<Size, Self::Error> {
        Ok(Size::new(self.cols, self.rows))
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<(), Self::Error> {
        self.cursor_visible = false;
        self.output.push_str("\x1b[?25l");
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<(), Self::Error> {
        self.cursor_visible = true;
        self.output.push_str("\x1b[?25h");
        Ok(())
    }

    fn get_cursor_position(&mut self) -> Result<Position, Self::Error> {
        Ok(self.cursor_pos)
    }

    fn set_cursor_position<P: Into<Position>>(&mut self, position: P) -> Result<(), Self::Error> {
        self.cursor_pos = position.into();
        self.cursor_visible = true;
        self.push_cursor_ansi();
        Ok(())
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn clear_region(&mut self, _clear_type: ClearType) -> Result<(), Self::Error> {
        Ok(())
    }

    fn window_size(&mut self) -> Result<WindowSize, Self::Error> {
        Ok(WindowSize {
            columns_rows: Size::new(self.cols, self.rows),
            pixels: Size::new(0, 0),
        })
    }

    fn draw<'a, I>(&mut self, content: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        let mut has_changes = false;
        for (x, y, cell) in content {
            if x < self.cols && y < self.rows {
                let c = cell.symbol().chars().next().unwrap_or(' ');
                let fg = cell.style().fg.unwrap_or(Color::Reset);
                let yi = y as usize;
                let xi = x as usize;
                if self.grid[yi][xi] != (c, fg) {
                    self.grid[yi][xi] = (c, fg);
                    has_changes = true;
                }
            }
        }

        if has_changes || !self.initialized {
            self.initialized = true;
            self.output.push_str(&self.render_full_grid());
        }

        Ok(())
    }
}
