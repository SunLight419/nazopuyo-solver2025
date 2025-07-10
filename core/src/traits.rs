use crate::types::{PuyoColor, Position, ChainInfo};

pub trait PuyoBoard {
    fn place_puyo(&mut self, position: Position, color: PuyoColor) -> Result<(), PlacementError>;
    
    fn get_puyo(&self, position: Position) -> Option<PuyoColor>;
    
    fn apply_gravity(&mut self);
    
    fn execute_chains(&mut self) -> ChainInfo;
    
    fn display(&self) -> String;
    
    fn is_game_over(&self) -> bool;
    
    fn clear(&mut self);
}

pub trait PuyoState {
    fn clone_state(&self) -> Box<dyn PuyoState>;
    
    fn hash_state(&self) -> u64;
    
    fn is_equivalent(&self, other: &dyn PuyoState) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlacementError {
    OutOfBounds,
    PositionOccupied,
    GameOver,
}

impl std::fmt::Display for PlacementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlacementError::OutOfBounds => write!(f, "Position is out of bounds"),
            PlacementError::PositionOccupied => write!(f, "Position is already occupied"),
            PlacementError::GameOver => write!(f, "Game is over"),
        }
    }
}

impl std::error::Error for PlacementError {}

pub trait PuyoPair {
    fn get_axis_puyo(&self) -> PuyoColor;
    fn get_child_puyo(&self) -> PuyoColor;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PuyoPairRotation {
    Up,
    Right,
    Down,
    Left,
}

pub trait PuyoPlacement {
    fn place_pair(&mut self, column: usize, rotation: PuyoPairRotation, pair: &dyn PuyoPair) -> Result<(), PlacementError>;
    
    fn get_valid_placements(&self, pair: &dyn PuyoPair) -> Vec<(usize, PuyoPairRotation)>;
}