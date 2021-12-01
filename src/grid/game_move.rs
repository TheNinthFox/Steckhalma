use crate::prelude::*;
use position::*;

pub enum MoveDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValidMove {
    pub origin: Position,
    pub middle: Position,
    pub target: Position,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidMoves {
    pub list: Vec<ValidMove>,
}