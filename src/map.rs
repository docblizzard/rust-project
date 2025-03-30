use rand::Rng;
use rand::thread_rng;
use std::io::{self, Write};
use std::{thread, time};
use crate::models::position::Position;

use crate::bfs::find_resource;

const MAP_WIDTH: usize = MAP[0].len();
const MAP_HEIGHT: usize = MAP.len();
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

pub fn map() {
    let map = load_map();
    let base = Position { x: 1, y: 1 };
    let mut robot = base;

    // 1. Find path to resource
    let Some(path_to_resource) = find_resource(&map, robot,MAP_WIDTH, MAP_HEIGHT, base, None) 
        else {
            println!("No path to resource found!");
        return;
    };

    // 2. Move to resource
    for step in &path_to_resource {
        print_map(&map, *step);
        robot = *step;
        thread::sleep(time::Duration::from_millis(800));
    }

    println!("✅ Reached resource at ({}, {})!", robot.x, robot.y);

    // 3. Return to base
    let Some(path_back) = find_resource(&map, robot,MAP_WIDTH, MAP_HEIGHT, base, Some(true)) 
        else {
            println!("Can't return to start!");
        return;
    };

    for step in &path_back {
        print_map(&map, *step);
        robot = *step;
        thread::sleep(time::Duration::from_millis(800));
    }

    println!("🏁 Returned to starting point ({}, {})!", base.x, base.y);
}

