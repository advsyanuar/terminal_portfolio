#[derive(Debug, Clone)]
pub struct Buffer {
    pub lines: Vec<String>,
    pub cursor_x: usize,
    pub cursor_y: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn from_text(text: &str) -> Self {
        let lines: Vec<String> = text.lines().map(|l| l.to_string()).collect();
        let lines = if lines.is_empty() {
            vec![String::new()]
        } else {
            lines
        };
        Self {
            lines,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn to_text(&self) -> String {
        self.lines.join("\n")
    }

    pub fn current_line(&self) -> &str {
        self.lines
            .get(self.cursor_y)
            .map(|s| s.as_str())
            .unwrap_or("")
    }

    fn current_line_mut(&mut self) -> &mut String {
        self.lines
            .get_mut(self.cursor_y)
            .expect("cursor_y out of bounds")
    }

    pub fn move_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.clamp_cursor_x();
        }
    }

    pub fn move_down(&mut self) {
        if self.cursor_y + 1 < self.lines.len() {
            self.cursor_y += 1;
            self.clamp_cursor_x();
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        } else if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.current_line().len();
        }
    }

    pub fn move_right(&mut self) {
        let line_len = self.current_line().len();
        if self.cursor_x < line_len {
            self.cursor_x += 1;
        } else if self.cursor_y + 1 < self.lines.len() {
            self.cursor_y += 1;
            self.cursor_x = 0;
        }
    }

    pub fn move_home(&mut self) {
        self.cursor_x = 0;
    }

    pub fn move_end(&mut self) {
        self.cursor_x = self.current_line().len();
    }

    pub fn move_page_up(&mut self, page_size: usize) {
        for _ in 0..page_size {
            if self.cursor_y == 0 {
                break;
            }
            self.cursor_y -= 1;
        }
        self.clamp_cursor_x();
    }

    pub fn move_page_down(&mut self, page_size: usize) {
        let max_y = self.lines.len().saturating_sub(1);
        for _ in 0..page_size {
            if self.cursor_y >= max_y {
                break;
            }
            self.cursor_y += 1;
        }
        self.clamp_cursor_x();
    }

    pub fn insert_char(&mut self, c: char) {
        let idx = self.cursor_x;
        let line = self.current_line_mut();
        line.insert(idx, c);
        self.cursor_x += 1;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_x > 0 {
            let idx = self.cursor_x - 1;
            let line = self.current_line_mut();
            line.remove(idx);
            self.cursor_x -= 1;
        } else if self.cursor_y > 0 {
            let prev_len = self.lines[self.cursor_y - 1].len();
            let current = self.lines.remove(self.cursor_y);
            self.cursor_y -= 1;
            self.lines[self.cursor_y].push_str(&current);
            self.cursor_x = prev_len;
        }
    }

    pub fn delete_forward(&mut self) {
        let line_len = self.current_line().len();
        if self.cursor_x < line_len {
            let idx = self.cursor_x;
            let line = self.current_line_mut();
            line.remove(idx);
        } else if self.cursor_y + 1 < self.lines.len() {
            let next = self.lines.remove(self.cursor_y + 1);
            self.lines[self.cursor_y].push_str(&next);
        }
    }

    pub fn split_line(&mut self) {
        let idx = self.cursor_x;
        let line = self.current_line_mut();
        let right = line.split_off(idx);
        self.lines.insert(self.cursor_y + 1, right);
        self.cursor_y += 1;
        self.cursor_x = 0;
    }

    pub fn join_line(&mut self) {
        if self.cursor_y > 0 && self.cursor_y < self.lines.len() {
            let prev_len = self.lines[self.cursor_y - 1].len();
            let current = self.lines.remove(self.cursor_y);
            self.cursor_y -= 1;
            self.lines[self.cursor_y].push_str(&current);
            self.cursor_x = prev_len;
        }
    }

    fn clamp_cursor_x(&mut self) {
        let max_x = self.current_line().len();
        if self.cursor_x > max_x {
            self.cursor_x = max_x;
        }
    }

    pub fn visible_line_range(&self, height: usize) -> (usize, usize) {
        let scroll_offset = if self.cursor_y >= height {
            self.cursor_y - height + 1
        } else {
            0
        };
        let end = (scroll_offset + height).min(self.lines.len());
        (scroll_offset, end)
    }
}
