pub mod mouse_input;
pub mod player_move;
pub mod process_mouse_input;
pub mod render;
pub mod setup;

use crate::prelude::*;

pub struct Materials {
    pub black: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub selected: Handle<ColorMaterial>,
    pub tile: Handle<ColorMaterial>,
    pub token: Handle<ColorMaterial>,
    pub valid: Handle<ColorMaterial>,
    pub inbetween: Handle<ColorMaterial>,
}
