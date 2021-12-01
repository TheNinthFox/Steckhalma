use crate::prelude::*;

pub fn render(
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
