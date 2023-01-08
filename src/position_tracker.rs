
pub struct Position {
    pub idx: usize,
    col: u64,
    ln: u64,
}

impl Position {
    pub fn new() -> Self {
        Self { col: 0, ln: 0, idx: 0 }
    }

    pub fn advance(&mut self, current_char: char) {
        self.idx += 1;
        self.col += 1;

        if '\n' == current_char {
            self.ln += 1;
            self.col = 0;
        }
    }

    pub fn get_position(&self) -> String {
        format!("{}:{}", self.ln, self.col)
    }
}
