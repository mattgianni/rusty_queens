// use env_logger;
use log::debug;

pub struct Board {
    columns: [i32; 8],
}

impl Board {
    pub fn new() -> Board {
        Board { columns: [0; 8] }
    }

    pub fn set(&mut self, col: i32, value: i32) {
        self.columns[col as usize] = value;
    }

    pub fn get(&self, col: i32) -> i32 {
        self.columns[col as usize]
    }

    pub fn print(&self) {
        for row in 0..8 {
            for col in 0..8 {
                if self.columns[col] == row as i32 {
                    print!("Q ");
                } else {
                    print!(". ");
                }
            }
            println!();
        }
    }

    pub fn place_piece(&mut self, column: i32) {
        debug!("placing piece in column {}", column);

        if column == 8 {
            println!("Solution Found!");
            self.print();
            return;
        }

        for r in 0..8 {
            let mut safe = true;

            for c in 0..column {
                if ((r - self.get(c)) == (column - c))
                    || ((self.get(c) - r) == (column - c))
                    || r == self.get(c)
                {
                    safe = false;
                    break;
                }
            }

            if safe {
                self.set(column, r);
                self.place_piece(column + 1);
            }
        }
    }

    pub fn print_all_solutions(&mut self) {
        debug!("placing the first piece");
        self.place_piece(0);
    }
}
