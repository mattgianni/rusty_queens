// use env_logger;
use log::debug;

pub struct Config {
    pub quiet: bool,
    pub num: usize,
}

pub struct Board {
    columns: Vec<usize>,
    available_rows: Vec<bool>,
    available_ldiag: Vec<bool>,
    available_rdiag: Vec<bool>,
    solutions: usize,
    config: Config,
}

impl Board {
    pub fn new(config: Config) -> Board {
        debug!("allocating new board");
        Board {
            columns: vec![0; config.num as usize],
            available_rows: vec![true; config.num as usize],
            available_ldiag: vec![true; (config.num * 2 - 1) as usize],
            available_rdiag: vec![true; (config.num * 2 - 1) as usize],
            solutions: 0,
            config,
        }
    }

    pub fn is_free(&self, column: usize, row: usize) -> bool {
        let rdiag = column + row;
        let ldiag = self.config.num - column + row - 1;
        self.available_rows[row] && self.available_rdiag[rdiag] && self.available_ldiag[ldiag]
    }

    pub fn place_piece(&mut self, column: usize, row: usize) {
        debug!("placing piece at column {} row {}", column, row);

        let rdiag = column + row;
        let ldiag = self.config.num - column + row - 1;

        assert!(self.available_rows[row]);
        assert!(self.available_rdiag[rdiag]);
        assert!(self.available_ldiag[ldiag]);

        self.columns[column] = row;
        self.available_rows[row] = false;
        self.available_rdiag[rdiag] = false;
        self.available_ldiag[ldiag] = false;
    }

    pub fn unplace_piece(&mut self, column: usize) {
        debug!("un-placing piece in column {}", column);

        let row = self.columns[column];
        debug!("the un-placed piece is in row {}", row);

        let rdiag = column + row;
        let ldiag = self.config.num - column + row - 1;

        assert!(!self.available_rows[row]);
        assert!(!self.available_rdiag[rdiag]);
        assert!(!self.available_ldiag[ldiag]);

        self.available_rows[row] = true;
        self.available_rdiag[rdiag] = true;
        self.available_ldiag[ldiag] = true;
    }

    fn find_spot(&mut self, column: usize) {
        if column == self.config.num {
            debug!("found a solution!");
            if !self.config.quiet {
                self.print();
            }
            self.solutions += 1;
        } else {
            for row in 0..self.config.num {
                if self.is_free(column, row) {
                    self.place_piece(column, row);
                    self.find_spot(column + 1);
                    self.unplace_piece(column);
                }
            }
        }
    }

    pub fn print(&self) {
        for row in 0..self.config.num {
            for col in 0..self.config.num {
                if self.columns[col as usize] == row {
                    print!("Q ");
                } else {
                    print!(". ");
                }
            }
            println!();
        }

        debug!("  avail rows: {:?}", self.available_rows);
        debug!(" avail rdiag: {:?}", self.available_rdiag);
        debug!(" avail ldiag: {:?}", self.available_ldiag);
    }

    pub fn print_all_solutions(&mut self) {
        debug!("printing all solutions");
        self.find_spot(0);
        println!("Found {} solutions.", self.solutions);
    }
}
