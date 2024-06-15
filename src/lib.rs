// use env_logger;
use log::debug;

pub struct Config {
    pub verbose: bool,
    pub quiet: bool,
    pub num: i32,
}

pub struct Board {
    columns: Vec<i32>,
    solutions: i32,
    config: Config,
}

impl Board {
    pub fn new(config: Config) -> Board {
        Board {
            columns: vec![0; config.num as usize],
            solutions: 0,
            config,
        }
    }

    pub fn set(&mut self, col: i32, value: i32) {
        self.columns[col as usize] = value;
    }

    pub fn get(&self, col: i32) -> i32 {
        self.columns[col as usize]
    }

    pub fn print(&self) {
        for row in 0..self.config.num {
            for col in 0..self.config.num {
                if self.columns[col as usize] == row as i32 {
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

        if column == self.config.num {
            self.solutions = self.solutions + 1;
            if !self.config.quiet {
                println!("");
                println!("Solution #{}", self.solutions);
                self.print();
            }
            return;
        }

        for r in 0..self.config.num {
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

        println!("Found {} solutions.", self.solutions);
    }
}
