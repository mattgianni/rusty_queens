use rusty_queens::Board;

#[test]
fn test_board() {
    let mut board = Board::new();
    board.set(0, 0, 1);
    assert_eq!(board.get(0, 0), 1);
}
