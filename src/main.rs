mod components;
mod events;
mod grid;
mod systems;

mod prelude {
    pub const WINDOW_SIZE: f32 = 500.0;

    pub use bevy::prelude::*;

    pub use crate::components::*;
    pub use crate::events::*;
    pub use crate::grid::*;

    pub use crate::systems::mouse_input::*;
    pub use crate::systems::player_move::*;
    pub use crate::systems::process_mouse_input::*;
    pub use crate::systems::render::*;
    pub use crate::systems::setup::*;
    pub use crate::systems::*;
}

use prelude::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Steckhalma".to_string(),
            width: WINDOW_SIZE,
            height: WINDOW_SIZE,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Grid::new())
        .add_event::<MouseDataEvent>()
        .add_event::<ValidMoveEvent>()
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(setup_board.system()))
        .add_system(mouse_input.system().label("mouse_input"))
        .add_system(
            process_mouse_input
                .system()
                .label("process_mouse_input")
                .after("mouse_input"),
        )
        .add_system(
            player_move
                .system()
                .label("handle_move")
                .after("process_mouse_input"),
        )
        .add_system(
            render
                .system()
                .label("update_visuals")
                .after("process_mouse_input"),
        )
        .add_plugins(DefaultPlugins)
        .run()
}
