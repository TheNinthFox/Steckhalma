use crate::prelude::*;

pub fn process_mouse_input(
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

        remove_highlights(
            &mut commands,
            &entity,
            valid.is_some(),
            inbetween.is_some(),
            hovered.is_some(),
            selected.is_some(),
            event.clicked,
        );

        update_hover_and_selection(
            &mut commands,
            &entity,
            &mouse_grid_pos,
            &tile,
            event.clicked,
        );

        check_move_and_highlights(
            &mut commands,
            &entity,
            &tile,
            &valid_moves,
            event.clicked,
            &mouse_grid_pos,
            &mut move_event_writer,
        );
    }
}

fn remove_highlights(
    commands: &mut Commands,
    entity: &Entity,
    valid: bool,
    inbetween: bool,
    hovered: bool,
    selected: bool,
    clicked: bool,
) {
    // Deselect valid move tiles
    if valid {
        commands.entity(*entity).remove::<ValidMoveComponent>();
    }

    // Deselect inbetween move tiles
    if inbetween {
        commands.entity(*entity).remove::<InbetweenComponent>();
    }

    // Remove hover from old tile
    if hovered {
        commands.entity(*entity).remove::<Hovered>();
    }

    // Remove Selection from old tile
    if clicked && selected {
        commands.entity(*entity).remove::<Selected>();
    }
}

fn update_hover_and_selection(
    commands: &mut Commands,
    entity: &Entity,
    mouse_grid_pos: &Position,
    tile: &Tile,
    clicked: bool,
) {
    // Mouse is over tile
    if *mouse_grid_pos == tile.position {
        // Hover
        commands.entity(*entity).insert(Hovered {});

        // If we have also clicked, select tile
        if clicked {
            commands.entity(*entity).insert(Selected {});
        }
    }
}

fn check_move_and_highlights(
    commands: &mut Commands,
    entity: &Entity,
    tile: &Tile,
    valid_moves: &Option<ValidMoves>,
    clicked: bool,
    mouse_grid_pos: &Position,
    move_event_writer: &mut EventWriter<ValidMoveEvent>,
) {
    // Move related logic
    match valid_moves {
        Some(moves) => {
            for valid_move in moves.list.iter() {
                if valid_move.target == tile.position {
                    // Highlight target
                    commands.entity(*entity).insert(ValidMoveComponent {});

                    // Make a move
                    if clicked && *mouse_grid_pos == tile.position {
                        move_event_writer.send(ValidMoveEvent {
                            data: valid_move.clone(),
                        });
                    }
                }

                // Highlight inbetween
                if valid_move.middle == tile.position {
                    commands.entity(*entity).insert(InbetweenComponent {});
                }
            }
        }
        None => (),
    }
}
