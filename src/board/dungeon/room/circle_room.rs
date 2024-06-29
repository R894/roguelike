use rand::{thread_rng, Rng};

use crate::vectors::Vector2Int;

use super::Room;

pub struct CircleRoom {
    radius: i32,
    center: Vector2Int,
}

impl CircleRoom {
    pub fn new(center: Vector2Int, radius: i32) -> CircleRoom {
        CircleRoom { center, radius }
    }
}

impl Room for CircleRoom {
    fn random_point(&self) -> Vector2Int {
        let mut rng = thread_rng();

        let x = rng.gen_range(self.center.x - self.radius..=self.center.x + self.radius);
        let y = rng.gen_range(self.center.y - self.radius..=self.center.y + self.radius);
        Vector2Int::new(x, y)
    }

    fn to_tiles(&self) -> std::collections::HashSet<Vector2Int> {
        let mut tiles = std::collections::HashSet::new();
        for x in self.center.x - self.radius..=self.center.x + self.radius {
            for y in self.center.y - self.radius..=self.center.y + self.radius {
                if (x - self.center.x).pow(2) + (y - self.center.y).pow(2) <= self.radius.pow(2) {
                    tiles.insert(Vector2Int::new(x, y));
                }
            }
        }
        tiles
    }

    fn centre(&self) -> Vector2Int {
        self.center
    }

    fn corners(&self) -> [Vector2Int; 4] {
        let mut corners = [Vector2Int::new(0, 0); 4];

        corners[0] = Vector2Int::new(self.center.x - self.radius, self.center.y - self.radius);
        corners[1] = Vector2Int::new(self.center.x + self.radius, self.center.y - self.radius);
        corners[2] = Vector2Int::new(self.center.x + self.radius, self.center.y + self.radius);
        corners[3] = Vector2Int::new(self.center.x - self.radius, self.center.y + self.radius);
        corners
    }

    fn intersects(&self, other: &dyn Room, border: Option<u32>) -> bool {
        let border_offset = border.unwrap_or(0) as i32;
        let other_center = other.centre();

        // check if the other room is within the circle radius
        let dx = other_center.x - self.center.x;
        let dy = other_center.y - self.center.y;
        if dx.pow(2) + dy.pow(2) <= (self.radius + border_offset).pow(2) {
            return true;
        }

        // if it isnt then check if the other room intersects the circle
        let other_tiles = other.to_tiles();
        for tile in other_tiles {
            let dx = tile.x - self.center.x;
            let dy = tile.y - self.center.y;
            if dx.pow(2) + dy.pow(2) <= (self.radius + border_offset).pow(2) {
                return true;
            }
        }

        false
    }

    fn shift(&mut self, offset: Vector2Int) {
        self.center += offset;
    }
}
