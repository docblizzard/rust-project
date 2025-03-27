use std::{collections::VecDeque, thread, time};
use crate::models::position::Position;


// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// struct Position {
//     x: usize,
//     y: usize,
// }

const WALL: char = '#';
const RESOURCE: char = '$';


fn is_valid(pos: Position, map: &Vec<Vec<char>>, visited: &Vec<Vec<bool>>, map_w: usize, map_h: usize) -> bool {
    pos.x < map_w
        && pos.y < map_h
        && map[pos.y][pos.x] != WALL
        && !visited[pos.y][pos.x]
}

pub fn find_resource(map: &Vec<Vec<char>>, start: Position, map_w: usize, map_h: usize) -> Option<Vec<Position>> {
    let directions = [
        (0, -1), // up
        (0, 1),  // down
        (-1, 0), // left
        (1, 0),  // right
    ];

    let mut visited = vec![vec![false; map_w]; map_h];
    let mut queue = VecDeque::new();
    let mut came_from = vec![vec![None; map_w]; map_h];

    visited[start.y][start.x] = true;
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        if map[current.y][current.x] == RESOURCE {
            let mut path = Vec::new();
            let mut pos = current;
            while pos != start {
                path.push(pos);
                pos = came_from[pos.y][pos.x].unwrap();
            }
            path.reverse();
            return Some(path);
        }
        // To improve?
        for (dx, dy) in directions {
            let new_x = current.x as isize + dx;
            let new_y = current.y as isize + dy;

            if new_x >= 0 && new_y >= 0 {
                let next = Position {
                    x: new_x as usize,
                    y: new_y as usize,
                };

                if is_valid(next, map, &visited, map_w, map_h) {
                    visited[next.y][next.x] = true;
                    came_from[next.y][next.x] = Some(current);
                    queue.push_back(next);
                }
            }
        }
    }

    None
}