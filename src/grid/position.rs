use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn to_index(&self) -> usize {
        self.row * GRID_SIZE + self.col
    }
}