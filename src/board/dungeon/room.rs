use std::collections::HashSet;

use rand::{thread_rng, Rng};

use crate::vectors::Vector2Int;
pub struct Room {
    pub a: Vector2Int,
    pub b: Vector2Int,
    pub walls: Vec<Vector2Int>,
}

impl Room {
    pub fn new(a: Vector2Int, b: Vector2Int) -> Room {
        let mut room = Room {
            a: Vector2Int::new(a.x.min(b.x), a.y.min(b.y)),
            b: Vector2Int::new(a.x.max(b.x), a.y.max(b.y)),
            walls: Vec::new(),
        };
        room.add_walls();
        room
    }

    pub fn corners(&self) -> [Vector2Int; 4] {
        [
            Vector2Int::new(self.a.x, self.a.y),
            Vector2Int::new(self.b.x, self.a.y),
            Vector2Int::new(self.b.x, self.b.y),
            Vector2Int::new(self.a.x, self.b.y),
        ]
    }

    fn add_walls(&mut self) {
        let min_x = self.a.x;
        let max_x = self.b.x;
        let min_y = self.a.y;
        let max_y = self.b.y;

        // Add walls for the perimeter of the room
        for x in min_x..=max_x {
            self.walls.push(Vector2Int::new(x, min_y)); // Top wall
            self.walls.push(Vector2Int::new(x, max_y)); // Bottom wall
        }
        for y in min_y..=max_y {
            self.walls.push(Vector2Int::new(min_x, y)); // Left wall
            self.walls.push(Vector2Int::new(max_x, y)); // Right wall
        }

        // Remove duplicate wall tiles
        self.walls.sort_unstable();
        self.walls.dedup();
    }

    pub fn random_point(&self) -> Vector2Int {
        let mut rng = thread_rng();
        let x = rng.gen_range(self.a.x + 1..self.b.x);
        let y = rng.gen_range(self.a.y + 1..self.b.y);
        Vector2Int::new(x, y)
    }

    pub fn to_tiles(&self) -> HashSet<Vector2Int> {
        (self.a.y..=self.b.y)
            .flat_map(|y| (self.a.x..=self.b.x).map(move |x| Vector2Int::new(x, y)))
            .collect()
    }
}
