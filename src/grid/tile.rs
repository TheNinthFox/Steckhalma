use crate::prelude::*;
use position::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Tile {
    pub position: Position,
    pub is_corner: bool,
    pub has_token: bool,
}