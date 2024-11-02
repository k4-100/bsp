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

// struct Section {
//     pub tl: (usize, usize),
//     pub br: (usize, usize),
// }

pub fn split_rooms(mut data_map: &mut [[&str; X_LENGTH]; Y_LENGTH]) {
    let mut rng = rand::thread_rng();
    let mut data_map_clone = data_map.clone();

    if rand::random() {
        let x: usize = rng.gen_range(20..X_LENGTH - 20);
        // check if split isn't to close to another
        let mut passed_check = true;
        let check_range = if x - 6 > 0 { x - 6 } else { 0 }..if x + 6 < X_LENGTH - 1 {
            x + 6
        } else {
            X_LENGTH - 1
        };

        for i in check_range {
            if data_map[0][i] == "#" {
                passed_check = false;
                break;
            }
        }

        // vertical
        if passed_check {
            let run_range = if rand::random() {
                0..data_map_clone.len()
            } else {
                data_map_clone.len() - 1..0
            };

            for i in run_range {
                // if data_map_clone[i][x] == "#" {
                //     break;
                // }

                if data_map_clone[i][x] == "#" {
                    if rand::random() {
                        data_map_clone = data_map.clone();
                    } else {
                        break;
                    }
                }
                data_map_clone[i][x] = "#";
            }
        }
    } else {
        let y: usize = rng.gen_range(6..Y_LENGTH - 6);

        // check if split isn't to close to another
        let mut passed_check = true;
        let check_range = if y - 3 > 0 { y - 3 } else { 0 }..if y + 3 < Y_LENGTH - 1 {
            y + 3
        } else {
            Y_LENGTH - 1
        };

        for i in check_range {
            if data_map[i][0] == "#" {
                passed_check = false;
                break;
            }
        }

        // horizontal

        if passed_check {
            let run_range = if rand::random() {
                0..data_map_clone[y].len()
            } else {
                data_map_clone[y].len() - 1..0
            };
            for i in run_range {
                if data_map_clone[y][i] == "#" {
                    if rand::random() {
                        data_map_clone = data_map.clone();
                    } else {
                        break;
                    }
                }
                data_map_clone[y][i] = "#";
            }
        }
    }

    *data_map = data_map_clone;
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
        thread::sleep(time::Duration::from_millis(20));
    }
}
