use crate::types::{PuyoColor, Position, ChainInfo};
use crate::traits::{PuyoBoard, PlacementError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleBitBoardPuyoBoard {
    columns: [u64; 6],
}

impl SimpleBitBoardPuyoBoard {
    pub fn new() -> Self {
        Self {
            columns: [0; 6],
        }
    }
    
    pub fn get_column_raw(&self, column: usize) -> u64 {
        if column >= 6 {
            0
        } else {
            self.columns[column]
        }
    }

    fn get_puyo_bits(&self, column: usize, row: usize) -> u8 {
        if column >= 6 || row >= 13 {
            return 0;
        }
        let shift = row * 3;
        ((self.columns[column] >> shift) & 0b111) as u8
    }

    fn set_puyo_bits(&mut self, column: usize, row: usize, color_bits: u8) {
        if column >= 6 || row >= 13 {
            return;
        }
        let shift = row * 3;
        let mask = 0b111u64 << shift;
        self.columns[column] = (self.columns[column] & !mask) | ((color_bits as u64) << shift);
    }

    pub fn get_column_height(&self, column: usize) -> usize {
        if column >= 6 {
            return 0;
        }
        
        let column_data = self.columns[column];
        if column_data == 0 {
            return 0;
        }
        
        let msb_index = 63 - column_data.leading_zeros();
        (msb_index as usize / 3) + 1
    }

    fn find_connected_group(&self, start_col: usize, start_row: usize, color: PuyoColor) -> Vec<Position> {
        let mut group = Vec::new();
        let mut visited = [[false; 13]; 6];
        let mut stack = vec![(start_col, start_row)];
        
        while let Some((col, row)) = stack.pop() {
            if col >= 6 || row >= 13 || visited[col][row] {
                continue;
            }
            
            if self.get_puyo_bits(col, row) != color.to_u8() {
                continue;
            }
            
            visited[col][row] = true;
            group.push(Position::new(col, row));
            
            if col > 0 {
                stack.push((col - 1, row));
            }
            if col < 5 {
                stack.push((col + 1, row));
            }
            if row > 0 {
                stack.push((col, row - 1));
            }
            if row < 12 {
                stack.push((col, row + 1));
            }
        }
        
        group
    }

    fn apply_gravity_to_column(&mut self, column: usize) {
        if column >= 6 {
            return;
        }
        
        let mut new_column = 0u64;
        let mut write_pos = 0;
        
        for read_pos in 0..13 {
            let puyo_bits = self.get_puyo_bits(column, read_pos);
            if puyo_bits != 0 {
                new_column |= (puyo_bits as u64) << (write_pos * 3);
                write_pos += 1;
            }
        }
        
        self.columns[column] = new_column;
    }
}

impl Default for SimpleBitBoardPuyoBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl PuyoBoard for SimpleBitBoardPuyoBoard {
    fn place_puyo(&mut self, position: Position, color: PuyoColor) -> Result<(), PlacementError> {
        if !position.is_valid_board_position() {
            return Err(PlacementError::OutOfBounds);
        }
        
        if self.get_puyo_bits(position.x, position.y) != 0 {
            return Err(PlacementError::PositionOccupied);
        }
        
        if self.is_game_over() {
            return Err(PlacementError::GameOver);
        }
        
        self.set_puyo_bits(position.x, position.y, color.to_u8());
        Ok(())
    }
    
    fn get_puyo(&self, position: Position) -> Option<PuyoColor> {
        if !position.is_valid_board_position() {
            return None;
        }
        
        let bits = self.get_puyo_bits(position.x, position.y);
        PuyoColor::from_u8(bits)
    }
    
    fn apply_gravity(&mut self) {
        for column in 0..6 {
            self.apply_gravity_to_column(column);
        }
    }
    
    fn execute_chains(&mut self) -> ChainInfo {
        let mut chain_info = ChainInfo::new();
        
        loop {
            let mut to_remove = Vec::new();
            
            for col in 0..6 {
                for row in 0..13 {
                    let puyo_bits = self.get_puyo_bits(col, row);
                    if puyo_bits == 0 || puyo_bits == 1 {
                        continue;
                    }
                    
                    let color = PuyoColor::from_u8(puyo_bits).unwrap();
                    if !color.is_colored_puyo() {
                        continue;
                    }
                    
                    let group = self.find_connected_group(col, row, color);
                    if group.len() >= 4 {
                        to_remove.extend(group);
                    }
                }
            }
            
            if to_remove.is_empty() {
                break;
            }
            
            for pos in to_remove {
                self.set_puyo_bits(pos.x, pos.y, 0);
            }
            
            chain_info.add_chain();
            self.apply_gravity();
        }
        
        chain_info
    }
    
    fn display(&self) -> String {
        let mut result = String::new();
        
        for row in (0..13).rev() {
            for col in 0..6 {
                let puyo_bits = self.get_puyo_bits(col, row);
                let char = match puyo_bits {
                    0 => '.',
                    1 => '#',
                    2 => 'R',
                    3 => 'B',
                    4 => 'G',
                    5 => 'Y',
                    6 => 'P',
                    _ => '?',
                };
                result.push(char);
            }
            result.push('\n');
        }
        
        result
    }
    
    fn is_game_over(&self) -> bool {
        self.get_puyo_bits(2, 11) != 0
    }
    
    fn clear(&mut self) {
        self.columns = [0; 6];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_board_is_empty() {
        let board = SimpleBitBoardPuyoBoard::new();
        assert_eq!(board.columns, [0; 6]);
    }
    
    #[test]
    fn test_place_and_get_puyo() {
        let mut board = SimpleBitBoardPuyoBoard::new();
        let pos = Position::new(0, 0);
        
        board.place_puyo(pos, PuyoColor::Red).unwrap();
        assert_eq!(board.get_puyo(pos), Some(PuyoColor::Red));
    }
    
    #[test]
    fn test_example_from_instruction() {
        let mut board = SimpleBitBoardPuyoBoard::new();
        
        board.place_puyo(Position::new(0, 0), PuyoColor::Red).unwrap();
        board.place_puyo(Position::new(0, 1), PuyoColor::Blue).unwrap();
        board.place_puyo(Position::new(0, 2), PuyoColor::Green).unwrap();
        
        let expected_bits = 0b100_011_010u64;
        assert_eq!(board.columns[0] & 0b111111111, expected_bits);
    }
    
    #[test]
    fn test_gravity() {
        let mut board = SimpleBitBoardPuyoBoard::new();
        
        board.place_puyo(Position::new(0, 5), PuyoColor::Red).unwrap();
        board.place_puyo(Position::new(0, 10), PuyoColor::Blue).unwrap();
        
        board.apply_gravity();
        
        assert_eq!(board.get_puyo(Position::new(0, 0)), Some(PuyoColor::Red));
        assert_eq!(board.get_puyo(Position::new(0, 1)), Some(PuyoColor::Blue));
        assert_eq!(board.get_puyo(Position::new(0, 5)), Some(PuyoColor::Empty));
        assert_eq!(board.get_puyo(Position::new(0, 10)), Some(PuyoColor::Empty));
    }
    
    #[test]
    fn test_column_height_optimization() {
        let mut board = SimpleBitBoardPuyoBoard::new();
        
        assert_eq!(board.get_column_height(0), 0);
        
        board.place_puyo(Position::new(0, 0), PuyoColor::Red).unwrap();
        assert_eq!(board.get_column_height(0), 1);
        
        board.place_puyo(Position::new(0, 1), PuyoColor::Blue).unwrap();
        assert_eq!(board.get_column_height(0), 2);
        
        board.place_puyo(Position::new(0, 2), PuyoColor::Green).unwrap();
        assert_eq!(board.get_column_height(0), 3);
        
        board.place_puyo(Position::new(0, 12), PuyoColor::Yellow).unwrap();
        assert_eq!(board.get_column_height(0), 13);
        
        assert_eq!(board.get_column_height(1), 0);
        
        board.place_puyo(Position::new(1, 5), PuyoColor::Purple).unwrap();
        assert_eq!(board.get_column_height(1), 6);
        
        assert_eq!(board.get_column_height(6), 0);
    }
}