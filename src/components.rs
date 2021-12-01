use crate::prelude::*;
use crate::grid::position::*;

pub struct BoardTile;
pub struct Hovered;
pub struct Selected;
pub struct InbetweenComponent;
pub struct ValidMoveComponent;

pub struct Token {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Token {
    pub fn from_position(position: &Position) -> Self {
        let translation = Grid::to_pixel(position);

        Self {
            x: translation.x,
            y: translation.y,
            width: TILE_SIZE_PADDED / 2.0,
            height: TILE_SIZE_PADDED / 2.0,
        }
    }
}
