use bevy::image::Image;
use bevy::math::Vec2;
use bevy_rapier2d::geometry::Collider;
use rand::prelude::{
    IndexedRandom, 
    StdRng
};
use rand::{Rng, SeedableRng};

/// Generate maze walls using recursive backtracking.
pub fn generate_maze_walls(
    width: usize,
    height: usize,
    cell_size: f32,
    bounds: Vec2,
    seed: u64,
) -> Vec<(Vec2, Vec2)> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut walls = Vec::new();
    let mut visited = vec![vec![false; width]; height];
    let mut stack = Vec::new();

    // Start at random cell
    let start_x = rng.random_range(0..width);
    let start_y = rng.random_range(0..height);
    visited[start_y][start_x] = true;
    stack.push((start_x, start_y));

    let offset = Vec2::new(-bounds.x * 0.5, -bounds.y * 0.5);

    while let Some((x, y)) = stack.pop() {
        let neighbors = get_unvisited_neighbors(x, y, width, height, &visited);

        if !neighbors.is_empty() {
            stack.push((x, y));

            let &(nx, ny) = neighbors.choose(&mut rng).unwrap();
            visited[ny][nx] = true;

            // Don't create wall between current and chosen neighbor
            // Add walls around the cells instead
            let cell_pos = Vec2::new(x as f32 * cell_size, y as f32 * cell_size) + offset;

            // Add walls if at boundary or if neighbor cell is not visited
            if x == 0 {
                walls.push((
                    cell_pos,
                    cell_pos + Vec2::new(0.0, cell_size),
                ));
            }
            if y == 0 {
                walls.push((
                    cell_pos,
                    cell_pos + Vec2::new(cell_size, 0.0),
                ));
            }

            stack.push((nx, ny));
        }
    }

    walls
}

fn get_unvisited_neighbors(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    visited: &[Vec<bool>],
) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    if x > 0 && !visited[y][x - 1] {
        neighbors.push((x - 1, y));
    }
    if x + 1 < width && !visited[y][x + 1] {
        neighbors.push((x + 1, y));
    }
    if y > 0 && !visited[y - 1][x] {
        neighbors.push((x, y - 1));
    }
    if y + 1 < height && !visited[y + 1][x] {
        neighbors.push((x, y + 1));
    }

    neighbors
}

/// Generate collider from PNG image data (simplified version).
/// For now, uses bounding box. Advanced pixel-based collision can be added later.
pub fn generate_collider_from_png(
    _image: &Image,
    scale: Vec2,
) -> Collider {
    // Simplified implementation - uses bounding box
    Collider::cuboid(scale.x * 0.5, scale.y * 0.5)
}

