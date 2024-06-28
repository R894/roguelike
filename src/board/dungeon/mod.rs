use std::collections::HashSet;

use crate::vectors::Vector2Int;

mod area;
pub mod room;
pub mod tunneler;

pub use area::Area;

const AREA_SPACING: i32 = 0;

pub struct Dungeon {
    pub areas: Vec<Area>,
    // the gird contains indexes to the areas vec
    // rows / columns
    grid: Vec<Vec<usize>>,
    pub walls: HashSet<Vector2Int>,
}

impl Dungeon {
    pub fn new(row_count: usize) -> Self {
        let grid = (0..row_count).map(|_| Vec::new()).collect::<Vec<_>>();
        Dungeon {
            areas: Vec::new(),
            grid,
            walls: HashSet::new(),
        }
    }

    pub fn add_area(&mut self, area: Area) {
        self.areas.push(area);
        let idx = self.areas.len() - 1;
        let row_count = self.grid.len();
        // insert the index to appropriate row vec
        self.grid[idx % row_count].push(idx);
    }

    pub fn generate(&mut self) {
        for area in self.areas.iter_mut() {
            area.generate_rooms();
        }
        self.position_areas();
        self.connect_areas();
        self.add_walls();
    }

    pub fn to_tiles(&self) -> HashSet<Vector2Int> {
        self.areas
            .iter()
            .flat_map(|a| a.to_tiles())
            .chain(self.walls.iter().copied())
            .collect()
    }

    fn position_areas(&mut self) {
        let column_count = self.grid[0].len();

        // calculate grid dimensions based on contained areas
        let column_widths = (0..column_count)
            .map(|i| {
                self.grid
                    .iter()
                    .map(|r| match r.get(i) {
                        None => 0,
                        Some(_a) => self.areas[i].get_size().x,
                    })
                    .max()
                    .unwrap()
                    + AREA_SPACING
            })
            .collect::<Vec<_>>();
        let row_heights = self
            .grid
            .iter()
            .map(|r| r.iter().map(|i| self.areas[*i].get_size().y).max().unwrap() + AREA_SPACING)
            .collect::<Vec<_>>();

        // calculate the offset amounts per each grid position
        let column_shifts = (0..column_widths.len())
            .map(|i| column_widths[..i].iter().sum())
            .collect::<Vec<i32>>();
        let row_shifts = (0..row_heights.len())
            .map(|i| row_heights[..i].iter().sum())
            .collect::<Vec<i32>>();

        // reposition the areas
        for (y, row) in self.grid.iter().enumerate() {
            for (x, idx) in row.iter().enumerate() {
                let offset = Vector2Int::new(column_shifts[x], row_shifts[y]);
                self.areas[*idx].shift(offset);
            }
        }
    }

    pub fn get_valid_spots(&self) -> Vec<Vector2Int> {
        self.areas.iter().flat_map(|a| a.to_tiles()).collect()
    }

    fn connect_areas(&mut self) {
        // connect areas based on their grid location
        let mut pairs = Vec::new();
        for (y, row) in self.grid.iter().enumerate() {
            for (x, idx) in row.iter().enumerate() {
                if x != 0 {
                    // join to area at x - 1
                    pairs.push((idx, row[x - 1]));
                };
                if y != 0 {
                    // join to area at y - 1
                    pairs.push((idx, self.grid[y - 1][x]));
                };
            }
        }
        for pair in pairs {
            let path = self.areas[*pair.0].join_area(&self.areas[pair.1]);
            self.areas[*pair.0].paths.borrow_mut().push(path);
        }
    }

    fn get_bounds(&self) -> (Vector2Int, Vector2Int) {
        let mut min = Vector2Int::new(i32::MAX, i32::MAX);
        let mut max = Vector2Int::new(i32::MIN, i32::MIN);
        for area in self.areas.iter() {
            let (a_min, a_max) = area.get_bounds();
            min.x = min.x.min(a_min.x);
            min.y = min.y.min(a_min.y);
            max.x = max.x.max(a_max.x);
            max.y = max.y.max(a_max.y);
        }
        (min, max)
    }

    fn add_walls(&mut self) {
        let (min, max) = self.get_bounds();
        for x in min.x - 1..=max.x + 1 {
            for y in min.y - 1..=max.y + 1 {
                let point = Vector2Int::new(x, y);
                let is_inside = self.areas.iter().any(|a| a.to_tiles().contains(&point));
                if !is_inside {
                    self.walls.insert(point);
                }
            }
        }
    }
}
