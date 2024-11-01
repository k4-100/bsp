use std::{
    collections::{BTreeMap, HashMap},
    thread, time,
};

use rand::Rng;

const X_LENGTH: usize = 120;
const Y_LENGTH: usize = 40;

#[derive(Debug)]
struct Cell {
    pub x: u8,
    pub y: u8,
}

impl Cell {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn to_map_key(&self) -> String {
        let Self { x, y } = self;

        format!("{x}-{y}")
    }
    // pub fn directly_to_map_key(x: u8, y: u8) -> String {
    //     format!("{x}-{y}")
    // }
    //
    // pub fn from_map_key(key: &str) -> Self {
    //     let vals: Vec<u8> = key
    //         .split("")
    //         .map(|number| number.parse().unwrap())
    //         .collect();
    //
    //     Self {
    //         x: vals[0],
    //         y: vals[1],
    //     }
    // }
}

pub fn split_rooms(mut data_map: &mut [[&str; X_LENGTH]; Y_LENGTH]) {
    let mut rng = rand::thread_rng();
    let mut vertical_split: bool = rand::random();

    if vertical_split {
        let x: usize = rng.gen_range(20..X_LENGTH - 20);

        for i in 0..data_map.len() {
            data_map[i][x] = "#";
        }

        return;
    }

    let y: usize = rng.gen_range(6..Y_LENGTH - 6);

    for i in 0..data_map[y].len() {
        data_map[y][i] = "#";
    }
}

fn main() {
    println!("\n");

    //setup

    let mut data_map: [[&str; X_LENGTH]; Y_LENGTH] = [["."; X_LENGTH]; Y_LENGTH];
    let mut i: usize = 0;

    //main execution loop
    loop {
        split_rooms(&mut data_map);
        let displayed_map: String = (0..Y_LENGTH)
            .map(|y| {
                let row: String = (0..X_LENGTH)
                    .map(|x| {
                        let block = data_map[y][x];
                        block
                    })
                    .collect();
                format!("{row}\n")
            })
            .collect();

        println!("iteration {i}");
        println!("{}", displayed_map);

        i += 1;
        thread::sleep(time::Duration::from_millis(1000));
    }
}
