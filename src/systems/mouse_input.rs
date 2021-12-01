use crate::prelude::*;

pub fn mouse_input(
    windows: Res<Windows>,
    mouse_button: Res<Input<MouseButton>>,
    mut mouse_data_writer: EventWriter<MouseDataEvent>,
) {
    let wnd = windows.get_primary().unwrap();
    if wnd.cursor_position().is_none() {
        return;
    }

    let position = wnd.cursor_position().unwrap();
    let normalized = Grid::ui_to_grid(position.x, position.y);

    let clicked = mouse_button.pressed(MouseButton::Left);

    mouse_data_writer.send(MouseDataEvent {
        pos: Vec2::new(normalized.x, normalized.y),
        clicked,
    })
}
