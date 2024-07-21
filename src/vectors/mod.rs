use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

mod utils;
pub use utils::find_path;

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub struct Vector2Int {
    pub x: i32,
    pub y: i32,
}

impl Vector2Int {
    pub const UP: Vector2Int = Vector2Int { x: 0, y: 1 };
    pub const DOWN: Vector2Int = Vector2Int { x: 0, y: -1 };
    pub const LEFT: Vector2Int = Vector2Int { x: -1, y: 0 };
    pub const RIGHT: Vector2Int = Vector2Int { x: 1, y: 0 };

    pub fn new(x: i32, y: i32) -> Vector2Int {
        Vector2Int { x, y }
    }

    pub fn manhattan(&self, other: Vector2Int) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn distance(&self, other: Vector2Int) -> i32 {
        let dx = (self.x - other.x).pow(2) as f64;
        let dy = (self.y - other.y).pow(2) as f64;
        (dx + dy).sqrt() as i32
    }

    pub fn circle_edge(&self, radius: i32) -> Vec<Vector2Int> {
        let mut tiles = Vec::new();
        for x in self.x - radius..=self.x + radius {
            for y in self.y - radius..=self.y + radius {
                if (x - self.x).pow(2) + (y - self.y).pow(2) == radius.pow(2) {
                    tiles.push(Vector2Int::new(x, y));
                }
            }
        }
        tiles
    }

    pub fn circle_area(&self, radius: i32) -> Vec<Vector2Int> {
        let mut tiles = Vec::new();
        for x in self.x - radius..=self.x + radius {
            for y in self.y - radius..=self.y + radius {
                if (x - self.x).pow(2) + (y - self.y).pow(2) <= radius.pow(2) {
                    tiles.push(Vector2Int::new(x, y));
                }
            }
        }
        tiles
    }
}

impl Add for Vector2Int {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector2Int::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Vector2Int {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for Vector2Int {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector2Int::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Vector2Int {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl Div<i32> for Vector2Int {
    type Output = Self;

    fn div(self, other: i32) -> Self {
        Vector2Int::new(self.x / other, self.y / other)
    }
}

impl Mul<i32> for Vector2Int {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Vector2Int::new(self.x * other, self.y * other)
    }
}

impl Mul<Vector2Int> for i32 {
    type Output = Vector2Int;

    fn mul(self, other: Vector2Int) -> Vector2Int {
        Vector2Int::new(other.x * self, other.y * self)
    }
}

pub const ORTHO_DIRECTIONS: [Vector2Int; 4] = [
    Vector2Int::UP,
    Vector2Int::DOWN,
    Vector2Int::LEFT,
    Vector2Int::RIGHT,
];

/// casts a line between two points then returns a vector of all the points between the source and the destination or until a blocker is found in the path
///
/// uses bresenham's line algorithm
pub fn cast_line(
    start: Vector2Int,
    end: Vector2Int,
    blocker_positions: &[Vector2Int],
) -> Vec<Vector2Int> {
    let mut path = Vec::new();
    let mut x0 = start.x;
    let mut y0 = start.y;
    let x1 = end.x;
    let y1 = end.y;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        // add the current point
        path.push(Vector2Int::new(x0, y0));

        // return if we've reached a blocker
        if blocker_positions.contains(&Vector2Int::new(x0, y0)) {
            return path;
        }

        // break if we've reached the end
        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }

    path
}

/// line of sight takes a start point and a list of perimeter points and returns a vector of the points in the path of the line of sight
pub fn line_of_sight(
    start: Vector2Int,
    area: Vec<Vector2Int>,
    blocker_positions: &[Vector2Int],
) -> Vec<Vector2Int> {
    let mut path = Vec::new();

    for point in area {
        path.extend(cast_line(start, point, blocker_positions));
    }

    path
}
