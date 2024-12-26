use std::{cell::RefCell, rc::Rc};

pub mod utils;

use utils::*;

fn main() {
    let displayed_grid = generate_map();

    displayed_grid.iter().for_each(|row| {
        row.iter().for_each(|cell| print!("{}", cell));
        println!();
    });
}
