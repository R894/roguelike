use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

use super::{Vector2Int, ORTHO_DIRECTIONS};

pub fn find_path(
    start: Vector2Int,
    end: Vector2Int,
    tiles: &HashSet<Vector2Int>,
    blockers: &HashSet<Vector2Int>,
) -> Option<VecDeque<Vector2Int>> {
    let mut queue = BinaryHeap::new();
    queue.push(Node { v: start, cost: 0 });
    let mut visited = HashMap::new();
    visited.insert(start, 0);
    let mut came_from = HashMap::new();

    while let Some(Node { v, cost }) = queue.pop() {
        if v == end {
            break;
        }
        for dir in ORTHO_DIRECTIONS {
            let n = v + dir;
            let new_cost = cost + 1;
            if !tiles.contains(&n) {
                continue;
            }
            // we allow the target to be a blocker
            if blockers.contains(&n) && n != end {
                continue;
            }
            match visited.get(&n) {
                Some(c) if *c <= new_cost => (),
                _ => {
                    visited.insert(n, new_cost);
                    queue.push(Node {
                        v: n,
                        cost: new_cost,
                    });
                    came_from.insert(n, v);
                }
            }
        }
    }
    let mut path = VecDeque::new();
    let mut cur = end;
    while let Some(v) = came_from.get(&cur) {
        path.push_front(cur);
        cur = *v;
        if cur == start {
            return Some(path);
        }
    }
    None
}

// helper struct for the path finder
#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    pub v: Vector2Int,
    pub cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.v.cmp(&other.v))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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
