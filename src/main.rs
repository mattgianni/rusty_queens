use rusty_queens::Board;

fn main() {
    let mut board = Board::new();

    board.set(0, 0, 1);
    println!("{}", board.get(0, 0));
}
