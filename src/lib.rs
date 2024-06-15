pub struct Board {
    columns: [[u8; 9]; 9],
}

impl Board {
    pub fn new() -> Board {
        Board {
            columns: [[0; 9]; 9],
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: u8) {
        self.columns[row][col] = value;
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.columns[row][col]
    }
}
