// use env_logger;
use log::debug;

pub struct Board {
    columns: [i32; 9],
}

impl Board {
    pub fn new() -> Board {
        Board { columns: [0; 9] }
    }

    pub fn set(&mut self, col: i32, value: i32) {
        self.columns[col as usize] = value;
    }

    pub fn get(&self, col: i32) -> i32 {
        self.columns[col as usize]
    }

    pub fn print(&self) {
        for col in 0..9 {
            print!("{} ", self.columns[col]);
        }
        println!();
    }

    pub fn print_full(&self) {
        for row in 0..9 {
            for col in 0..9 {
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
            self.print_full();
            return;
        }

        for row in 0..9 {
            let mut safe = true;
            for i in 0..column {
                if ((row - self.get(i)) == (column - i))
                    && ((self.get(i) - row) == (column - i))
                    && row == self.get(i)
                {
                    safe = false;
                    break;
                }
            }

            if safe {
                self.set(column, row);
                self.place_piece(column + 1);
            }
        }
    }

    pub fn print_all_solutions(&mut self) {
        debug!("placing the first piece");
        self.place_piece(0);
    }
}
