use rand::Rng;
use rand::thread_rng;
use std::io::{self, Write};


const MAP_WIDTH: usize = 10;
const MAP_HEIGHT: usize = 10;
const WALL: char = '#';
const EMPTY: char = '.';


fn generate_map() -> Vec<Vec<char>> {
    let mut map = vec![vec![EMPTY; MAP_WIDTH]; MAP_HEIGHT];

    let mut rng = rand::thread_rng();
    for i in 0..MAP_HEIGHT {
        for j in 0..MAP_WIDTH {
            if rng.gen_range(0..10) < 3 {
                map[i][j] = WALL;
            }
        }
    }

    map
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

pub fn map() {
    let map = generate_map();
    print_map(&map);
}

