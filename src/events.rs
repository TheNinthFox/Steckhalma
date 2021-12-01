use crate::prelude::*;

pub struct MouseDataEvent {
    pub pos: Vec2,
    pub clicked: bool,
}

pub struct ValidMoveEvent {
    pub data: ValidMove,
}
