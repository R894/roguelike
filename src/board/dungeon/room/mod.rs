mod circle_room;
mod square_room;

use std::collections::HashSet;

use circle_room::CircleRoom;
use rand::prelude::*;
use square_room::SquareRoom;

use crate::vectors::Vector2Int;

pub trait RoomGenerator {
    fn generate(&self) -> GeneratorResult;
}

pub struct GeneratorResult {
    pub rooms: Vec<Box<dyn Room>>,
    pub connections: Vec<(usize, usize)>,
}

pub struct BubbleGenerator {
    // bounds for a random room count
    pub room_count: (u32, u32),
    // min max room size
    pub room_size: (u32, u32),
    // min distance between rooms
    pub room_padding: Option<u32>,
    pub extra_connection_chance: f64,
}
impl BubbleGenerator {
    fn random_dim(&self) -> (i32, i32) {
        let mut rng = thread_rng();
        (
            rng.gen_range(self.room_size.0..=self.room_size.1) as i32,
            rng.gen_range(self.room_size.0..=self.room_size.1) as i32,
        )
    }

    fn generate_room(&self, min_corner: Vector2Int, max_corner: Vector2Int) -> Box<dyn Room> {
        let mut rng = thread_rng();
        if rng.gen_bool(0.5) {
            Box::new(SquareRoom::new(min_corner, max_corner))
        } else {
            Box::new(CircleRoom::new(
                min_corner,
                rng.gen_range(self.room_size.0 as i32..=self.room_size.1 as i32),
            ))
        }
    }
}
impl RoomGenerator for BubbleGenerator {
    fn generate(&self) -> GeneratorResult {
        let mut rng = thread_rng();
        let mut connections = Vec::new();

        let (w, h) = self.random_dim();
        let mut rooms = vec![self.generate_room(Vector2Int::new(0, 0), Vector2Int::new(w, h))];
        let count = rng.gen_range(self.room_count.0..=self.room_count.1);

        let max_dist = self.room_size.1 as i32;

        for _ in 0..=count {
            loop {
                // Randomly select an existing room as a base for the new room
                let prev_idx = rng.gen_range(0..rooms.len());

                // pick a random point around prev's centre
                let centre = rooms[prev_idx].centre();
                let a = Vector2Int::new(
                    rng.gen_range(centre.x - max_dist..=centre.x + max_dist),
                    rng.gen_range(centre.y - max_dist..=centre.y + max_dist),
                );

                // get random room size
                let (w, h) = self.random_dim();
                // get a second corner in a random direction
                let b = Vector2Int::new(
                    a.x + *[-w, w].choose(&mut rng).unwrap(),
                    a.y + *[-h, h].choose(&mut rng).unwrap(),
                );

                let new_room = self.generate_room(a, b);
                // Check for intersections with existing rooms
                if rooms
                    .iter()
                    .any(|other| new_room.intersects(&**other, self.room_padding))
                {
                    continue;
                }

                // Add connection between this room and the selected previous room
                connections.push((prev_idx, rooms.len()));

                // Optionally add an extra random connection
                if rng.gen_bool(self.extra_connection_chance) {
                    connections.push((rng.gen_range(0..rooms.len()), rooms.len()));
                }

                // Add the new room to the list
                rooms.push(new_room);

                // Break the loop for successful room placement
                break;
            }
        }

        GeneratorResult { rooms, connections }
    }
}

pub trait Room {
    fn random_point(&self) -> Vector2Int;
    fn to_tiles(&self) -> HashSet<Vector2Int>;
    fn centre(&self) -> Vector2Int;
    fn corners(&self) -> [Vector2Int; 4];
    fn intersects(&self, other: &dyn Room, border: Option<u32>) -> bool;
    fn shift(&mut self, offset: Vector2Int);
}
