use crate::prelude::*;

pub fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
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

pub fn setup_board(mut commands: Commands, grid: Res<Grid>, materials: Res<Materials>) {
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
