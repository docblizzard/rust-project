use rand::Rng;
use rand::thread_rng;
use std::io::{self, Write};
use std::{thread, time};
use crate::models::position::Position;

use crate::bfs::find_resource;

const MAP_WIDTH: usize = MAP[0].len();
const MAP_HEIGHT: usize = MAP.len();
const WALL: char = '#';
const RESOURCE: char = '$';
const ROBOT: char = 'R';
const MAP: [&str; 9] = [
    "##########",
    "#........#",
    "#.....####",
    "##......##",
    "#....$..##",
    "#........#",
    "#...#...##",
    "#........#",
    "##########",
];

// #[derive(Clone, Copy)]
// struct Position {
//     x: usize,
//     y: usize,
// }

fn load_map() -> Vec<Vec<char>> {
    MAP.iter().map(|line| line.chars().collect()).collect()
}


fn print_map(map: &Vec<Vec<char>>, robot_pos: Position) {
    print!("\x1B[2J\x1B[1;1H");
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            if x == robot_pos.x && y == robot_pos.y {
                print!("{}", ROBOT);
            } else {
                print!("{}", map[y][x]);
            }
        }
        println!();
    }
}

// fn try_move(map: &Vec<Vec<char>>, robot: &mut Position) {
//     let mut rng = thread_rng();
//     let directions = [
//         (0, -1), // up
//         (0, 1),  // down
//         (-1, 0), // left
//         (1, 0),  // right
//     ];

//     for _ in 0..10 {
//         let (dx, dy) = directions[rng.gen_range(0..4)];
//         let new_x = robot.x as isize + dx;
//         let new_y = robot.y as isize + dy;

//         if new_x >= 0 && new_x < MAP_WIDTH as isize &&
//            new_y >= 0 && new_y < MAP_HEIGHT as isize {
//             let new_x = new_x as usize;
//             let new_y = new_y as usize;

//             if map[new_y][new_x] != WALL {
//                 robot.x = new_x;
//                 robot.y = new_y;
//                 break;
//             }
//         }
//     }
// }


pub fn map() {
    let map = load_map();

    let mut robot = Position { x: 1, y: 1 }; 

    let Some(path) = find_resource(&map, robot,MAP_WIDTH, MAP_HEIGHT) else {
        println!("path not found");
        return;
    };

    for step in path {
        print_map(&map, step);
        robot = step;
        thread::sleep(time::Duration::from_millis(900));
    }

    println!("Robot found resource: ({}, {})", robot.x, robot.y);
}

