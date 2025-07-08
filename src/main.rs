mod bitboard;

fn main() {
    let mut board = bitboard::Board::default();
    board.set_xy(0, 1, 1);
    board.set_xy(0, 2, 3);
    board.set_xy(1, 2, 4);
    println!("{}", board);
}
