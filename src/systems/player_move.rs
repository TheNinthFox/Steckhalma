use crate::prelude::*;

pub fn player_move(
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
