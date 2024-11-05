use std::{
    collections::{BTreeMap, HashMap},
    thread, time,
};

use rand::Rng;

const X_LENGTH: usize = 120;
const Y_LENGTH: usize = 40;

// fn calculate_sections(mut data_map: &mut [[&str; X_LENGTH]; Y_LENGTH]) -> Vec<Section> {
//     let mut sections: Vec<Section> = Vec::new();
//     let mut section_count = 0;
//
//     println!("section_count: {section_count}");
//
//     sections
// }

// pub fn split_rooms(mut data_map: &mut [[&str; X_LENGTH]; Y_LENGTH]) {
//     let mut rng = rand::thread_rng();
//     let mut data_map_clone = data_map.clone();
//
//     // let sections = calculate_sections(data_map);
//
//     if rand::random() {
//         let x: usize = rng.gen_range(20..X_LENGTH - 20);
//
//         // vertical
//         let run_range = 0..data_map_clone.len();
//
//         for i in run_range {
//             data_map[i][x] = "#";
//         }
//     } else {
//         let y: usize = rng.gen_range(6..Y_LENGTH - 6);
//
//         // horizontal
//         let run_range = 0..data_map_clone[y].len();
//         for i in run_range {
//             data_map[y][i] = "#";
//         }
//     }
// }

struct Section {
    pub tl: (usize, usize),
    pub br: (usize, usize),
}

enum SplitVariant {
    Horizontal,
    Vertical,
}

// struct BTree {
//     pub parent: Option<*mut BTree>,
//     pub children: [Option<*mut BTree>; 2],
//     pub data: Box<Section>,
// }

// struct BTree {
//     pub parent: Option<*mut BTree>,
//     pub children: [Option<*mut BTree>; 2],
//     pub data: Box<Section>,
// }
#[derive(Clone, Debug)]
struct BTree {
    // pub parent: Option<*mut BTree>,
    pub children: [Option<Box<BTree>>; 2],
    // pub data: Box<Section>,
    pub data: Box<usize>,
}

impl BTree {
    pub fn new(i: usize) -> Self {
        Self {
            children: [None, None],
            data: Box::new(i),
        }
    }

    pub fn new_with_children(i: usize, children: [BTree; 2]) -> Self {
        Self {
            children: [
                Some(Box::new(children[0].clone())),
                Some(Box::new(children[1].clone())), //
            ],
            data: Box::new(i),
        }
    }

    pub fn traverse_with_stuff(&self, recursion_count: &mut usize) {
        *recursion_count += 1;
        println!("recursion_count: {recursion_count}");
        for child_option in self.children.iter() {
            if let Some(child) = child_option {
                child.traverse_with_stuff(recursion_count);
            }
        }
    }

    // pub fn traverse(&self) {
    //     println!("recursion_count: {recursion_count}");
    //     for child_option in self.children.iter() {
    //         if let Some(child) = child_option {
    //             child.traverse();
    //         }
    //     }
    // }

    pub fn reach_leaves(&self, leaves: &mut Vec<Box<BTree>>) {
        for child_option in self.children.iter() {
            if let Some(child) = child_option {
                if child.children[0].is_none() && child.children[1].is_none() {
                    leaves.push(child.clone());
                }
                child.reach_leaves(leaves);
            }
        }
    }
}

fn main() {
    let mut recursion_count = 0;
    // let mut btree: BTree = BTree::new(0);

    let mut btree: BTree = BTree::new_with_children(
        0,
        [
            BTree::new_with_children(1, [BTree::new(2), BTree::new(3)]),
            BTree::new_with_children(4, [BTree::new(5), BTree::new(6)]),
        ],
    );

    // btree.traverse_with_stuff(&mut recursion_count);
    let mut v_btree: Vec<Box<BTree>> = Vec::new();

    btree.reach_leaves(&mut v_btree);

    println!("{:#?}", v_btree);

    //setup

    // let mut data_map: [[&str; X_LENGTH]; Y_LENGTH] = [["."; X_LENGTH]; Y_LENGTH];
    // let mut i: usize = 0;
    //
    // //main execution loop
    // loop {
    //     // split_rooms(&mut data_map);
    //     let displayed_map: String = (0..Y_LENGTH)
    //         .map(|y| {
    //             let row: String = (0..X_LENGTH)
    //                 .map(|x| {
    //                     let block = data_map[y][x];
    //                     block
    //                 })
    //                 .collect();
    //             format!("{row}\n")
    //         })
    //         .collect();
    //
    //     println!("iteration {i}");
    //     println!("{}", displayed_map);
    //
    //     i += 1;
    //     thread::sleep(time::Duration::from_millis(1000));
    // }
}
