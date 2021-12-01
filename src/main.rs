mod components;
mod grid;

mod prelude {
    pub const WINDOW_SIZE: f32 = 500.0;

    pub use bevy::prelude::*;

    pub use crate::components::*;
    pub use crate::grid::*;
}

use prelude::*;

struct Materials {
    black: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    selected: Handle<ColorMaterial>,
    tile: Handle<ColorMaterial>,
    token: Handle<ColorMaterial>,
    valid: Handle<ColorMaterial>,
    inbetween: Handle<ColorMaterial>,
}

struct MouseDataEvent {
    pos: Vec2,
    clicked: bool,
}

struct ValidMoveEvent {
    data: ValidMove,
}

fn handle_move(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    mut move_reader: EventReader<ValidMoveEvent>,
    tokens: Query<(Entity, &Transform), With<Token>>,
) {
    let move_reader = move_reader.iter().next();
    if move_reader.is_none() {
        return;
    }

    let move_made = move_reader.unwrap().data;
    grid.update_tile(&move_made.origin, false, false);
    grid.update_tile(&move_made.middle, false, false);
    grid.update_tile(&move_made.target, false, true);

    for (entity, transform) in tokens.iter() {
        let normalized = Grid::world_to_grid(transform.translation);
        let position = Grid::from_pixel(normalized.x, normalized.y);

        if move_made.origin == position {
            let mut pixel_position = Grid::to_pixel(&move_made.target);
            pixel_position.z = 1.0;

            commands
                .entity(entity)
                .insert(Transform::from_translation(pixel_position));
        }

        if move_made.middle == position {
            commands.entity(entity).despawn();
        }
    }
}

fn update_visuals(
    mut commands: Commands,
    grid: Res<Grid>,
    mats: Res<Materials>,
    entities: Query<
        (
            Entity,
            &Transform,
            Option<&Selected>,
            Option<&Hovered>,
            Option<&ValidMoveComponent>,
            Option<&InbetweenComponent>,
        ),
        With<BoardTile>,
    >,
) {
    for (entity, transform, selected, hovered, valid, inbetween) in entities.iter() {
        let normalized = Grid::world_to_grid(transform.translation);
        let tile = grid.tile_from_pixel(normalized.x, normalized.y);
        let selected = selected.is_some();
        let hovered = hovered.is_some();
        let valid = valid.is_some();
        let inbetween = inbetween.is_some();

        let color = match (selected, hovered, valid, tile, inbetween) {
            // Selected
            (selected, _, _, tile, _) if selected && tile.has_token => mats.selected.clone(),
            // Hovered
            (selected, hovered, _, tile, _) if !selected && hovered && !tile.is_corner => {
                mats.hovered.clone()
            }
            // Inbetween Move
            (selected, _, valid, tile, inbetween)
                if !selected && !valid && tile.has_token && inbetween =>
            {
                mats.inbetween.clone()
            }
            // Valid Move
            (selected, _, valid, tile, _) if !selected && valid && !tile.has_token => {
                mats.valid.clone()
            }
            // Tile
            (selected, hovered, valid, tile, _)
                if !selected && !hovered && !valid && !tile.is_corner =>
            {
                mats.tile.clone()
            }
            // Corner
            (_, _, _, tile, _) if tile.is_corner => mats.black.clone(),
            _ => mats.token.clone(),
        };

        commands.entity(entity).insert(color);
    }
}

fn process_mouse_input(
    mut commands: Commands,
    mut mouse_data_reader: EventReader<MouseDataEvent>,
    mut move_event_writer: EventWriter<ValidMoveEvent>,
    grid: Res<Grid>,
    selected: Query<(Entity, &Transform), With<Selected>>,
    board_tiles: Query<
        (
            Entity,
            &Transform,
            Option<&Hovered>,
            Option<&Selected>,
            Option<&ValidMoveComponent>,
            Option<&InbetweenComponent>,
        ),
        With<BoardTile>,
    >,
) {
    let mouse_data_reader = mouse_data_reader.iter().next();
    if mouse_data_reader.is_none() {
        return;
    }

    let event = mouse_data_reader.unwrap();
    let selected_entity = selected.iter().next();
    let mouse_pos = event.pos;
    let mouse_grid_pos = Grid::from_pixel(mouse_pos.x, mouse_pos.y);

    let mut valid_moves: Option<ValidMoves> = None;
    if selected_entity.is_some() {
        let (_, selected_transform) = selected_entity.unwrap();
        let normalized = Grid::world_to_grid(selected_transform.translation);
        let tile = grid.tile_from_pixel(normalized.x, normalized.y);
        valid_moves = Some(grid.calculate_valid_moves(&tile.position));
    }

    for (entity, transform, hovered, selected, valid, inbetween) in board_tiles.iter() {
        let normalized = Grid::world_to_grid(transform.translation);
        let tile = grid.tile_from_pixel(normalized.x, normalized.y).clone();

        // Deselect valid move tiles
        if valid.is_some() {
            commands.entity(entity).remove::<ValidMoveComponent>();
        }

        // Deselect inbetween move tiles
        if inbetween.is_some() {
            commands.entity(entity).remove::<InbetweenComponent>();
        }

        // Highlight valid move tiles
        match &valid_moves {
            Some(moves) => {
                for valid_move in moves.list.iter() {
                    if valid_move.target == tile.position {
                        commands.entity(entity).insert(ValidMoveComponent {});

                        if event.clicked && mouse_grid_pos == tile.position {
                            move_event_writer.send(ValidMoveEvent {
                                data: valid_move.clone(),
                            });
                        }
                    }

                    if valid_move.middle == tile.position {
                        commands.entity(entity).insert(InbetweenComponent {});
                    }
                }
            }
            None => (),
        }

        // Mouse is over tile
        if mouse_grid_pos == tile.position {
            // Hover
            commands.entity(entity).insert(Hovered {});

            // If we have also clicked, select tile
            if event.clicked {
                commands.entity(entity).insert(Selected {});
            }
        // Mouse is not over tile
        } else {
            // Remove hover from old tile
            if hovered.is_some() {
                commands.entity(entity).remove::<Hovered>();
            }

            // Remove Selection from old tile
            if event.clicked && selected.is_some() {
                commands.entity(entity).remove::<Selected>();
            }
        }
    }
}

fn mouse(
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

fn setup_board(mut commands: Commands, grid: Res<Grid>, materials: Res<Materials>) {
    for tile in grid.tiles.iter() {
        let mat = if tile.is_corner {
            materials.black.clone()
        } else {
            materials.tile.clone()
        };

        // Spawn board tiles.
        commands
            .spawn_bundle(SpriteBundle {
                material: mat,
                sprite: Sprite::new(Vec2::new(TILE_SIZE_PADDED, TILE_SIZE_PADDED)),
                transform: Transform::from_translation(Grid::to_pixel(&tile.position)),
                ..Default::default()
            })
            .insert(BoardTile {});

        if !tile.has_token {
            continue;
        }

        // Spawn tokens.
        let token = Token::from_position(&tile.position);
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.token.clone(),
                sprite: Sprite::new(Vec2::new(token.width, token.height)),
                transform: Transform::from_translation(Vec3::new(token.x, token.y, 1.0)),
                ..Default::default()
            })
            .insert(token);
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        black: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
        hovered: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
        selected: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
        tile: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
        token: materials.add(Color::rgb(0.75, 0.0, 0.0).into()),
        valid: materials.add(Color::rgb(1.0, 1.0, 0.0).into()),
        inbetween: materials.add(Color::rgb(1.0, 0.5, 0.0).into()),
    });
}

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
        .add_system(mouse.system().label("mouse"))
        .add_system(
            process_mouse_input
                .system()
                .label("process_mouse_input")
                .after("mouse"),
        )
        .add_system(
            handle_move
                .system()
                .label("handle_move")
                .after("process_mouse_input"),
        )
        .add_system(
            update_visuals
                .system()
                .label("update_visuals")
                .after("process_mouse_input"),
        )
        .add_plugins(DefaultPlugins)
        .run()
}
