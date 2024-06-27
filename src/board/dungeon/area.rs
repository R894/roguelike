use std::{cell::RefCell, collections::HashSet};

use crate::vectors::Vector2Int;

use super::{room::Room, tunneler::Tunneler};

pub struct Area {
    pub rooms: Vec<Room>,
    pub paths: RefCell<Vec<Vec<Vector2Int>>>,
    pub tunneler: Box<dyn Tunneler>,
    pub walls: RefCell<HashSet<Vector2Int>>,
}
impl Area {
    pub fn new(tunneler: Box<dyn Tunneler>) -> Self {
        Area {
            rooms: Vec::new(),
            paths: RefCell::new(Vec::new()),
            tunneler,
            walls: RefCell::new(HashSet::new()),
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

        let mut new_walls = HashSet::new();
        for &w in self.walls.borrow().iter() {
            new_walls.insert(w + d);
        }
        *self.walls.borrow_mut() = new_walls;
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
        self.update_walls_along_path(&path);
        other.update_walls_along_path(&path);

        path
    }

    pub fn update_walls_along_path(&self, path: &[Vector2Int]) {
        for point in path {
            if self.walls.borrow().contains(point) {
                self.walls.borrow_mut().remove(point);
            }
        }
    }

    pub fn generate_rooms(&mut self) {
        self.rooms = vec![
            Room::new(Vector2Int::new(0, 0), Vector2Int::new(4, 6)),
            Room::new(Vector2Int::new(10, 2), Vector2Int::new(14, 8)),
        ];
        self.paths = vec![self.join_rooms(&self.rooms[0], &self.rooms[1])].into();
        self.add_walls();
        self.remove_path_obstacles();
    }

    pub fn to_tiles(&self) -> HashSet<Vector2Int> {
        self.rooms
            .iter()
            .flat_map(|r| r.to_tiles())
            .chain(self.paths.borrow().iter().flatten().copied())
            .chain(self.walls.borrow().iter().copied())
            .collect()
    }

    pub fn add_walls(&mut self) {
        let bounds = self.get_bounds();
        let min_x = bounds.0.x;
        let max_x = bounds.1.x;
        let min_y = bounds.0.y;
        let max_y = bounds.1.y;

        let occupied_tiles: HashSet<Vector2Int> = self.to_tiles();

        for room in self.rooms.iter() {
            for &wall in room.walls.iter() {
                self.walls.borrow_mut().insert(wall);
            }
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let tile = Vector2Int::new(x, y);
                if !occupied_tiles.contains(&tile) {
                    self.walls.borrow_mut().insert(tile);
                }
            }
        }
    }

    pub fn remove_path_obstacles(&mut self) {
        for path in self.paths.borrow().iter() {
            for point in path {
                if self.walls.borrow().contains(point) {
                    self.walls.borrow_mut().remove(point);
                }
            }
        }
    }
}
