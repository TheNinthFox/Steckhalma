use super::*;

#[test]
fn test_ui_to_grid() {
    assert_eq!(Grid::ui_to_grid(0.0, 500.0), Vec3::new(0.0, 0.0, 0.0));
    assert_eq!(Grid::ui_to_grid(500.0, 500.0), Vec3::new(500.0, 0.0, 0.0));
    assert_eq!(Grid::ui_to_grid(250.0, 250.0), Vec3::new(250.0, 250.0, 0.0));
}

#[test]
fn test_world_to_grid() {
    assert_eq!(Grid::world_to_grid(Vec3::new(0.0, 0.0, 0.0)), Vec3::new(250.0, 250.0, 0.0));
    assert_eq!(Grid::world_to_grid(Vec3::new(250.0, 250.0, 0.0)), Vec3::new(500.0, 0.0, 0.0));
    assert_eq!(Grid::world_to_grid(Vec3::new(500.0, 500.0, 0.0)), Vec3::new(750.0, -250.0, 0.0));
}

#[test]
fn test_is_corner() {
    assert_eq!(Grid::is_corner(&Position {row: 0, col: 0}), true);
    assert_eq!(Grid::is_corner(&Position {row: 0, col: 5}), true);
    assert_eq!(Grid::is_corner(&Position {row: 5, col: 0}), true);
    assert_eq!(Grid::is_corner(&Position {row: 5, col: 5}), true);
    assert_eq!(Grid::is_corner(&Position {row: 3, col: 3}), false);
}

#[test]
fn test_from_index() {
    assert_eq!(Grid::from_index(0), Position {row: 0, col: 0});
    assert_eq!(Grid::from_index(6), Position {row: 0, col: 6});
    assert_eq!(Grid::from_index(24), Position {row: 3, col: 3});
    assert_eq!(Grid::from_index(42), Position {row: 6, col: 0});
    assert_eq!(Grid::from_index(48), Position {row: 6, col: 6});
}

#[test]
fn test_from_pixel() {
    assert_eq!(Grid::from_pixel(0.0, 0.0), Position {row: 0, col: 0});
    assert_eq!(Grid::from_pixel(250.0, 0.0), Position {row: 0, col: 3});
    assert_eq!(Grid::from_pixel(499.0, 0.0), Position {row: 0, col: 6});
    assert_eq!(Grid::from_pixel(0.0, 250.0), Position {row: 3, col: 0});
    assert_eq!(Grid::from_pixel(0.0, 499.0), Position {row: 6, col: 0});
    assert_eq!(Grid::from_pixel(250.0, 250.0), Position {row: 3, col: 3});
}

#[test]
fn test_to_pixel() {
    assert_eq!(Grid::to_pixel(&Position {row: 0, col: 0}), Vec3::new(-214.2857, 214.2857, 0.0));
    assert_eq!(Grid::to_pixel(&Position {row: 0, col: 6}), Vec3::new(214.28574, 214.2857, 0.0));
    assert_eq!(Grid::to_pixel(&Position {row: 3, col: 3}), Vec3::new(0.000015258789, -0.000015258789, 0.0));
    assert_eq!(Grid::to_pixel(&Position {row: 6, col: 0}), Vec3::new(-214.2857, -214.28574, 0.0));
    assert_eq!(Grid::to_pixel(&Position {row: 6, col: 6}), Vec3::new(214.28574, -214.28574, 0.0));
}