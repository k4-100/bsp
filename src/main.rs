use std::{cell::RefCell, rc::Rc};

pub mod utils;

use utils::*;

fn main() {
    let mut tree = TreeNode::new_with_children(
        Section::new((1, 1), (X_LENGTH - 1, Y_LENGTH - 1)),
        None,
        None,
    );
    let mut tree_ref = Rc::new(RefCell::new(tree));
    let mut leaves = TreeNode::reach_leaves(tree_ref.clone());

    for _ in 0..100 {
        TreeNode::split_leaves(leaves.clone());
        leaves = TreeNode::reach_leaves(tree_ref.clone());
    }
    println!("leaves:\n{:#?}", leaves.len());

    let mut displayed_grid: Vec<Vec<&str>> = (0..Y_LENGTH)
        .map(|_| (0..X_LENGTH).map(|_| "#").collect::<Vec<&str>>())
        .collect();

    for y in 0..Y_LENGTH {
        for x in 0..X_LENGTH {
            if leaves.len() != 0 {
                for leaf in leaves.iter() {
                    let leaf_unwrapped = &leaf.borrow();
                    if leaf_unwrapped.data.contains((x, y)) == true {
                        displayed_grid[y][x] = ".";
                    }
                }
            }

            print!("{}", displayed_grid[y][x]);
        }
        println!("");
    }
}
