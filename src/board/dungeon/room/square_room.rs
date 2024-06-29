use std::collections::HashSet;

use rand::{thread_rng, Rng};

use crate::vectors::Vector2Int;

use super::Room;

pub struct SquareRoom {
    pub a: Vector2Int,
    pub b: Vector2Int,
}

impl SquareRoom {
    pub fn new(a: Vector2Int, b: Vector2Int) -> SquareRoom {
        SquareRoom {
            a: Vector2Int::new(a.x.min(b.x), a.y.min(b.y)),
            b: Vector2Int::new(a.x.max(b.x), a.y.max(b.y)),
        }
    }
}

impl Room for SquareRoom {
    fn corners(&self) -> [Vector2Int; 4] {
        [
            Vector2Int::new(self.a.x, self.a.y),
            Vector2Int::new(self.b.x, self.a.y),
            Vector2Int::new(self.b.x, self.b.y),
            Vector2Int::new(self.a.x, self.b.y),
        ]
    }

    fn random_point(&self) -> Vector2Int {
        let mut rng = thread_rng();
        let x = rng.gen_range(self.a.x + 1..self.b.x);
        let y = rng.gen_range(self.a.y + 1..self.b.y);
        Vector2Int::new(x, y)
    }

    fn to_tiles(&self) -> HashSet<Vector2Int> {
        (self.a.y..=self.b.y)
            .flat_map(|y| (self.a.x..=self.b.x).map(move |x| Vector2Int::new(x, y)))
            .collect()
    }

    fn centre(&self) -> Vector2Int {
        Vector2Int::new((self.b.x + self.a.x) / 2, (self.b.y + self.a.y) / 2)
    }

    fn intersects(&self, other: &dyn Room, border: Option<u32>) -> bool {
        let b = border.unwrap_or(0) as i32;

        let other_tiles = other.to_tiles();

        for y in self.a.y - b..=self.b.y + b {
            for x in self.a.x - b..=self.b.x + b {
                if other_tiles.contains(&Vector2Int::new(x, y)) {
                    return true;
                }
            }
        }
        false
    }

    fn shift(&mut self, offset: Vector2Int) {
        self.a += offset;
        self.b += offset;
    }
}
