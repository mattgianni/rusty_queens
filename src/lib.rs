pub struct Board {
    columns: [u8; 9],
}

impl Board {
    pub fn new() -> Board {
        Board { columns: [0; 9] }
    }

    pub fn set(&mut self, col: usize, value: u8) {
        self.columns[col] = value;
    }

    pub fn get(&self, col: usize) -> u8 {
        self.columns[col]
    }

    pub fn print(&self) {
        for col in 0..9 {
            print!("{} ", self.columns[col]);
        }
        println!();
    }
}
