use core::{PuyoColor, Position, PuyoBoard, SimpleBitBoardPuyoBoard};

fn main() {
    println!("謎ぷよソルバー2025");
    println!("BitBoard implementation demo:");
    
    let mut board = SimpleBitBoardPuyoBoard::new();
    
    board.place_puyo(Position::new(0, 0), PuyoColor::Red).unwrap();
    board.place_puyo(Position::new(0, 1), PuyoColor::Blue).unwrap();
    board.place_puyo(Position::new(0, 2), PuyoColor::Green).unwrap();
    
    println!("Example from instruction (red/blue/green column):");
    println!("Raw bits: {:09b}", board.get_column_raw(0) & 0b111111111);
    println!("Expected:  100011010");
    println!("Column height: {}", board.get_column_height(0));
    
    println!("\nBoard display:");
    print!("{}", board.display());
    
    println!("Performance demo - Column heights:");
    for col in 0..6 {
        println!("Column {}: height {}", col, board.get_column_height(col));
    }
}
