#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PuyoColor {
    Empty = 0,
    Garbage = 1,
    Red = 2,
    Blue = 3,
    Green = 4,
    Yellow = 5,
    Purple = 6,
}

impl PuyoColor {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Empty),
            1 => Some(Self::Garbage),
            2 => Some(Self::Red),
            3 => Some(Self::Blue),
            4 => Some(Self::Green),
            5 => Some(Self::Yellow),
            6 => Some(Self::Purple),
            _ => None,
        }
    }
    
    pub fn to_u8(self) -> u8 {
        self as u8
    }
    
    pub fn is_colored_puyo(self) -> bool {
        matches!(self, Self::Red | Self::Blue | Self::Green | Self::Yellow | Self::Purple)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    
    pub fn is_valid_board_position(self) -> bool {
        self.x < 6 && self.y < 13
    }
    
    pub fn is_game_over_position(self) -> bool {
        self.x == 2 && self.y == 11
    }
}

#[derive(Debug, Clone)]
pub struct ChainInfo {
    pub chain_count: usize,
}

impl ChainInfo {
    pub fn new() -> Self {
        Self {
            chain_count: 0,
        }
    }
    
    pub fn add_chain(&mut self) {
        self.chain_count += 1;
    }
}

impl Default for ChainInfo {
    fn default() -> Self {
        Self::new()
    }
}