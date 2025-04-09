use rand::Rng;
use rand::thread_rng;
use std::io::{self, Write};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::{thread, time};
use crate::models::position::Position;

use crate::bfs::find_resource;
use crate::models::robot::GameState;
use crate::models::robot::Robot;

const MAP_WIDTH: usize = MAP[0].len();
const MAP_HEIGHT: usize = MAP.len();
const ROBOT: char = 'R';
const RESOURCE: char = '$';
const MAP: [&str; 9] = [
    "####################",
    "#B.................#",
    "#.........#######..#",
    "##........#.......##",
    "#........#..#.....##",
    "#...........#$.....#",
    "#............#..#..#",
    "#..................#",
    "####################",
];

fn load_map() -> Vec<Vec<char>> {
    MAP.iter().map(|line| line.chars().collect()).collect()
}


fn print_map(map: &Vec<Vec<char>>, robots: &Vec<Robot>) {
    print!("\x1B[2J\x1B[1;1H");
    
    let mut display = map.clone();
    
    for robot in robots {
        display[robot.position.y][robot.position.x] = ROBOT;
    }
    
    for row in &display {
        for &cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

pub fn run_threads() {
    let base = Position { x: 1, y: 1 };
    let map = load_map();
    
    // Mutexe
    let state = Arc::new(Mutex::new(GameState {
        map,
        robots: Vec::new(),
        finished_robots: 0,
    }));
    
    // Robots creation
    let robot_count = 3;
    
    {
        let mut game_state = state.lock().unwrap();
        for id in 0..robot_count {
            game_state.robots.push(Robot {
                id,
                position: base,
                has_resource: false,
            });
        }
    }
    
    // Terminal Display
    let display_state = Arc::clone(&state);
    let display_handle = thread::spawn(move || {
        let mut all_finished = false;
        
        while !all_finished {

            let game_state = display_state.lock().unwrap();
            print_map(&game_state.map, &game_state.robots);

            all_finished = game_state.finished_robots >= robot_count;
            drop(game_state);

            thread::sleep(Duration::from_millis(100));
        }
        
        println!("Finished");
    });
    
    let mut robot_handles = Vec::new();
    for robot_id in 0..robot_count {
        let robot_state = Arc::clone(&state);
        let handle = thread::spawn(move || {

            thread::sleep(Duration::from_millis(1000 * robot_id as u64));
            robot_task(robot_id, robot_state, base);
        });

        robot_handles.push(handle);
    }
    
    for handle in robot_handles {
        handle.join().unwrap();
    }
    display_handle.join().unwrap();
}


fn robot_task(robot_id: usize, state: Arc<Mutex<GameState>>, base: Position) {
    let mut has_resource = false;
    
    loop {
        // Get current position and find bfs path 
        let (start_pos, target_path) = {

            let game_state = state.lock().unwrap();
            let robot = &game_state.robots[robot_id];
            
            let path = if !has_resource {

                find_resource(&game_state.map, robot.position, MAP_WIDTH, MAP_HEIGHT, base, None)
            } else {

                find_resource(&game_state.map, robot.position, MAP_WIDTH, MAP_HEIGHT, base, Some(true))
            };
            
            (robot.position, path)
        };
        
        // retry
        let Some(path) = target_path else {

            thread::sleep(Duration::from_millis(500));
            continue;
        };
        
        for &step in &path {

            let reached_resource;
            let returned_to_base;
            {
                let game_state = state.lock().unwrap();
                reached_resource = !has_resource && game_state.map[step.y][step.x] == RESOURCE;
                returned_to_base = has_resource && step == base;
            }
            
            {
                let mut game_state = state.lock().unwrap();
                let robot = &mut game_state.robots[robot_id];
                robot.position = step;
                
                if reached_resource {
                    has_resource = true;
                    robot.has_resource = true;
                    println!(" Robot {} found resource at : ({}, {})", robot_id, step.x, step.y);
                }
                
                if returned_to_base {
                    game_state.finished_robots += 1;
                    return;
                }
            }
            thread::sleep(Duration::from_millis(800));
        }
    }
}



