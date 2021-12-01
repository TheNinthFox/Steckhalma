use crate::prelude::*;

pub const GRID_SIZE: usize = 7;
pub const TILE_SIZE: f32 = WINDOW_SIZE as f32 / GRID_SIZE as f32;
pub const PADDING: f32 = 10.0;
pub const TILE_SIZE_PADDED: f32 = TILE_SIZE - PADDING;
pub const CENTER: usize = 24;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Tile {
    pub position: Position,
    pub is_corner: bool,
    pub has_token: bool,
}

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

pub struct Grid {
    pub tiles: Vec<Tile>,
}

impl Grid {
    pub fn new() -> Self {
        let mut tiles = Vec::new();

        for index in 0..(GRID_SIZE * GRID_SIZE) {
            let position = Grid::from_index(index);
            let is_corner = Grid::is_corner(&position);

            tiles.push(Tile {
                position: position,
                is_corner,
                has_token: !is_corner && index != CENTER,
            });
        }

        Self { tiles }
    }

    pub fn tile_from_position(&self, position: &Position) -> &Tile {
        &self.tiles[position.to_index()]
    }

    pub fn tile_from_pixel(&self, x: f32, y: f32) -> &Tile {
        let position = Grid::from_pixel(x, y);
        self.tile_from_position(&position)
    }

    pub fn calculate_valid_moves(&self, origin: &Position) -> ValidMoves {
        let mut moves = Vec::new();
        let mut options = Vec::new();

        options.push(self.calculate_valid_move(origin, MoveDirection::Left));
        options.push(self.calculate_valid_move(origin, MoveDirection::Right));
        options.push(self.calculate_valid_move(origin, MoveDirection::Up));
        options.push(self.calculate_valid_move(origin, MoveDirection::Down));
        for option in options.iter() {
            match option {
                Some(valid_move) => moves.push(*valid_move),
                None => (),
            }
        }

        ValidMoves { list: moves }
    }

    pub fn calculate_valid_move(
        &self,
        origin: &Position,
        direction: MoveDirection,
    ) -> Option<ValidMove> {
        let mut middle = origin.clone();
        let mut target = origin.clone();

        match direction {
            MoveDirection::Left => {
                if origin.col < 2 {
                    return None;
                }

                middle.col -= 1;
                target.col -= 2;
            }
            MoveDirection::Right => {
                if origin.col > (GRID_SIZE - 3) {
                    return None;
                }

                middle.col += 1;
                target.col += 2;
            }
            MoveDirection::Up => {
                if origin.row < 2 {
                    return None;
                }

                middle.row -= 1;
                target.row -= 2;
            }
            MoveDirection::Down => {
                if origin.row > (GRID_SIZE - 3) {
                    return None;
                }

                middle.row += 1;
                target.row += 2;
            }
        }

        if !self.is_valid_move(&origin, &middle, &target) {
            return None;
        }

        Some(ValidMove {
            origin: origin.clone(),
            middle,
            target,
        })
    }

    pub fn is_valid_move(&self, origin: &Position, middle: &Position, target: &Position) -> bool {
        let origin_tile = self.tile_from_position(origin);
        let middle_tile = self.tile_from_position(middle);
        let target_tile = self.tile_from_position(target);

        origin_tile.has_token
            && !origin_tile.is_corner
            && middle_tile.has_token
            && !middle_tile.is_corner
            && !target_tile.has_token
            && !target_tile.is_corner
    }

    pub fn update_tile(&mut self, position: &Position, is_corner: bool, has_token: bool) {
        self.tiles[position.to_index()].is_corner = is_corner;
        self.tiles[position.to_index()].has_token = has_token;
    }

    pub fn to_pixel(position: &Position) -> Vec3 {
        let x = -(WINDOW_SIZE as f32 / 2.0) + (TILE_SIZE / 2.0) + (position.col as f32 * TILE_SIZE);
        let y = (WINDOW_SIZE as f32 / 2.0) - (TILE_SIZE / 2.0) - (position.row as f32 * TILE_SIZE);

        Vec3::new(x, y, 0.0)
    }

    pub fn from_pixel(x: f32, y: f32) -> Position {
        Position {
            row: (y / TILE_SIZE).floor() as usize,
            col: (x / TILE_SIZE).floor() as usize,
        }
    }

    pub fn from_index(index: usize) -> Position {
        Position {
            row: index / GRID_SIZE,
            col: index % GRID_SIZE,
        }
    }

    pub fn is_corner(position: &Position) -> bool {
        let row = position.row;
        let col = position.col;

        // Top Left
        (row < 2 && col < 2)
        // Top Right
        || (row > GRID_SIZE-3 && col < 2)
        // Bottom Left
        || (row < 2 && col > GRID_SIZE-3)
        // Bottom RRight
        || (row > GRID_SIZE-3 && col > GRID_SIZE-3)
    }

    pub fn world_to_grid(translation: Vec3) -> Vec3 {
        Vec3::new(
            translation.x + WINDOW_SIZE / 2.0,
            WINDOW_SIZE / 2.0 - translation.y,
            0.0,
        )
    }

    pub fn ui_to_grid(x: f32, y: f32) -> Vec3 {
        Vec3::new(x, WINDOW_SIZE - y, 0.0)
    }
}
