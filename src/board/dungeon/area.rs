use std::{cell::RefCell, collections::HashSet};

use crate::vectors::Vector2Int;

use super::{
    room::{Room, RoomGenerator},
    tunneler::Tunneler,
};

pub struct Area {
    pub rooms: Vec<Room>,
    pub paths: RefCell<Vec<Vec<Vector2Int>>>,
    pub tunneler: Box<dyn Tunneler>,
    pub room_generator: Box<dyn RoomGenerator>,
}
impl Area {
    pub fn new(tunneler: Box<dyn Tunneler>, room_generator: Box<dyn RoomGenerator>) -> Self {
        Area {
            rooms: Vec::new(),
            paths: RefCell::new(Vec::new()),
            tunneler,
            room_generator,
        }
    }

    pub fn get_bounds(&self) -> (Vector2Int, Vector2Int) {
        let min_x = self.rooms.iter().map(|r| r.a.x).min().unwrap();
        let max_x = self.rooms.iter().map(|r| r.b.x).max().unwrap();
        let min_y = self.rooms.iter().map(|r| r.a.y).min().unwrap();
        let max_y = self.rooms.iter().map(|r| r.b.y).max().unwrap();
        (Vector2Int::new(min_x, min_y), Vector2Int::new(max_x, max_y))
    }

    pub fn get_size(&self) -> Vector2Int {
        let bounds = self.get_bounds();
        Vector2Int::new(bounds.1.x - bounds.0.x + 1, bounds.1.y - bounds.0.y + 1)
    }

    pub fn shift(&mut self, offset: Vector2Int) {
        // translate the entire area by a given offset
        let bounds = self.get_bounds();
        let d = offset - bounds.0;

        for room in self.rooms.iter_mut() {
            room.a += d;
            room.b += d;
        }
        for path in self.paths.borrow_mut().iter_mut() {
            for v in path.iter_mut() {
                *v += d;
            }
        }
    }

    pub fn join_rooms(&self, a: &Room, b: &Room) -> Vec<Vector2Int> {
        self.tunneler.connect(a.random_point(), b.random_point())
    }

    fn find_closest_room_pair<'a>(&'a self, other: &'a Area) -> (&'a Room, &'a Room) {
        // find closest room pair between two areas
        // based on corner distances only
        let mut pairs = Vec::new();
        for ra in self.rooms.iter() {
            for rb in other.rooms.iter() {
                // find min corner dist
                let d = ra
                    .corners()
                    .iter()
                    .flat_map(|ca| {
                        rb.corners()
                            .iter()
                            .map(|cb| ca.manhattan(*cb))
                            .collect::<Vec<_>>()
                    })
                    .min()
                    .unwrap();
                pairs.push((d, ra, rb));
            }
        }
        // sort by corner dist
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        (pairs[0].1, pairs[0].2)
    }

    pub fn join_area(&self, other: &Area) -> Vec<Vector2Int> {
        let (room_self, room_other) = self.find_closest_room_pair(other);
        let path = self.join_rooms(room_self, room_other);

        // Update self's paths and walls
        self.paths.borrow_mut().push(path.clone());
        path
    }

    pub fn generate_rooms(&mut self) {
        let result = self.room_generator.generate();
        self.rooms = result.rooms;

        self.paths.borrow_mut().clear();
        for connection in &result.connections {
            let path = self.join_rooms(&self.rooms[connection.0], &self.rooms[connection.1]);
            self.paths.borrow_mut().push(path);
        }
    }

    pub fn to_tiles(&self) -> HashSet<Vector2Int> {
        self.rooms
            .iter()
            .flat_map(|r| r.to_tiles())
            .chain(self.paths.borrow().iter().flatten().copied())
            .collect()
    }
}
