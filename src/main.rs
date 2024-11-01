use std::{
    collections::{BTreeMap, HashMap},
    thread, time,
};
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

// pub fn split_rooms()

const Y_LENGTH: usize = 40;
const X_LENGTH: usize = 120;

fn main() {
    println!("\n");

    // setup

    let mut data_map: [[&str; X_LENGTH]; Y_LENGTH] = [["."; X_LENGTH]; Y_LENGTH];
    let mut i: usize = 0;

    // main execution loop
    loop {
        thread::sleep(time::Duration::from_millis(1000));
        println!("iteration {i}");

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

        println!("{}", displayed_map);
        i += 1;
    }
}
