use std::{
    collections::{BTreeMap, HashMap},
    thread, time,
};

use rand::{random, Rng};

const X_LENGTH: usize = 120;
const Y_LENGTH: usize = 40;

// fn calculate_sections(mut data_map: &mut [[&str; X_LENGTH]; Y_LENGTH]) -> Vec<Section> {
//     let mut sections: Vec<Section> = Vec::new();
//     let mut section_count = 0;
//
//     prinltn!("section_count: {section_count}");
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

#[derive(Clone, Copy, Debug)]
struct Section {
    pub lt: (usize, usize),
    pub rb: (usize, usize),
}

impl Section {
    pub fn new(lt: (usize, usize), rb: (usize, usize)) -> Self {
        Self { lt, rb }
    }

    pub fn contains(&self, point: (usize, usize)) -> bool {
        (self.lt.0 <= point.0 && point.0 < self.rb.0)
            && (self.lt.1 <= point.1 && point.1 < self.rb.1)
    }
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
    pub children: Vec<Option<BTree>>,
    pub data: Section,
}

impl BTree {
    pub fn new(data: Section) -> Self {
        Self {
            children: vec![None, None],
            data,
        }
    }

    pub fn new_with_children(data: Section, children: [Option<BTree>; 2]) -> Self {
        Self {
            children: vec![children[0].clone(), children[1].clone()],
            data,
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
    //     prinltn!("recursion_count: {recursion_count}");
    //     for child_option in self.children.iter() {
    //         if let Some(child) = child_option {
    //             child.traverse();
    //         }
    //     }
    // }

    // pub fn reach_leaves(&self, leaves: &mut Vec<&Box<BTree>>) {
    //     for child_option in self.children.iter() {
    //         if let Some(child) = child_option {
    //             if child.children[0].is_none() && child.children[1].is_none() {
    //                 // leaves.push(child.clone());
    //                 leaves.push(child);
    //             }
    //             child.reach_leaves(leaves);
    //         }
    //     }
    //
    //     // if there were no children, add tree node to leaves
    //     if leaves.is_empty() {
    //         leaves.push(Box::new(self.clone()));
    //     }
    // }

    pub fn reach_leaves<'a>(&'a self, leaves: &mut Vec<&'a Option<BTree>>) {
        for child in self.children.iter() {
            if let Some(child_unwrapped) = child {
                if child_unwrapped.children[0].is_none() && child_unwrapped.children[1].is_none() {
                    leaves.push(child);
                    self.reach_leaves(leaves);
                }
            }
        }

        // if leaves.is_empty() {
        //
        //     // leaves.push(Some(self));
        //     // leaves.push(Some(self));
        // }
    }

    // pub fn reach_leaves<'a>(&'a self, leaves: &mut Vec<&'a BTree>) {
    //     for child_option in self.children.iter() {
    //         if let Some(child) = child_option {
    //             if !child.children[0].is_none() && !child.children[1].is_none() {
    //                 // leaves.push(child.clone());
    //                 leaves.push(child);
    //             }
    //             child.reach_leaves(leaves);
    //         }
    //     }
    //
    //     // if there were no children, add tree node to leaves
    //     if leaves.is_empty() {
    //         leaves.push(self);
    //     }
    // }

    // pub fn reach_leaves<'a>(&'a mut self, leaves: &mut Vec<&mut BTree>) {
    //     {
    //         for child_option in self.children.iter_mut() {
    //             if let Some(child) = child_option {
    //                 if child.children[0].is_none() && child.children[1].is_none() {
    //                     // leaves.push(child.clone());
    //                     leaves.push(*child);
    //                 }
    //                 // child.reach_leaves(leaves);
    //             }
    //         }
    //     }
    //
    //     // if there were no children, add tree node to leaves
    //     if leaves.is_empty() {
    //         // leaves.push(self);
    //     }
    // }

    // pub fn reach_leaves(&self, mut leaves: Vec<Box<BTree>>) {
    //     for child_option in self.children.iter() {
    //         if let Some(child) = child_option {
    //             if child.children[0].is_none() && child.children[1].is_none() {
    //                 leaves.push(child);
    //             }
    //             child.reach_leaves(leaves);
    //         }
    //     }
    //
    //     // if there were no children, add tree node to leaves
    //     if leaves.is_empty() {
    //         leaves.push(Box::new(self.clone()));
    //     }
    // }
    //
    //     pub fn split_leaves(&self) {
    //         let mut leaves: Vec<&BTree> = Vec::new();
    //         self.reach_leaves(&mut leaves);
    //         let mut mutable_leaves: Vec<BTree> = Vec::with_capacity(leaves.capacity());
    //
    //         for leaf in &mut leaves {
    //             let mut single_leaf = leaf.clone();
    //             for i in 0..=1 {
    //                 if let Some(mut divided_leaf) = single_leaf.children[i].clone() {
    //                     let Section { lt, rb } = divided_leaf.data;
    //                     let divide: usize;
    //                     let mut rng = rand::thread_rng();
    //                     // horizontal split - pick some y point and split horizontally
    //                     if random() {
    //                         divide = rng.gen_range(lt.1 + 2..rb.1 - 2);
    //
    //                         divided_leaf.children[0] =
    //                             Some(BTree::new(Section::new((lt.0, lt.1), (rb.0, divide - 1))));
    //                     }
    //                     single_leaf.children[i] = Some(divided_leaf);
    //                 }
    //             }
    //             mutable_leaves.push(single_leaf);
    //         }
    //     }
    // }
}

fn main() {
    let mut sections = BTree::new(Section::new((1, 1), (X_LENGTH - 1, Y_LENGTH - 1)));
    let mut leaves: Vec<&Option<BTree>> = Vec::new();

    // sections.split_leaves();
    // sections.split_leaves();

    sections.reach_leaves(&mut leaves);

    println!("leaves:\n{:#?}", leaves);

    let mut displayed_grid: Vec<Vec<&str>> = (0..Y_LENGTH)
        .map(|_| (0..X_LENGTH).map(|_| "*").collect::<Vec<&str>>())
        .collect();

    for y in 0..Y_LENGTH {
        for x in 0..X_LENGTH {
            if leaves.len() != 0 {
                for leave in leaves.iter() {
                    if leave.clone().clone().unwrap().data.contains((x, y)) != true {
                        // displayed_grid[y][x] = ".";
                        displayed_grid[y][x] = "#";
                    }
                    // else {
                    //     displayed_grid[y][x] = "#";
                    // }
                }
            }
            // for leave in leaves.iter(){}

            print!("{}", displayed_grid[y][x]);
        }
        println!("");
    }
}

// let mut recursion_count = 0;
// let mut btree: BTree = BTree::new(0);

// let mut btree: BTree = BTree::new(Section::new((0, 0), (X_LENGTH, Y_LENGTH)));

// let mut btree: BTree = BTree::new_with_children(
//     Section::new((0, 0), (X_LENGTH, Y_LENGTH)),
//     [
//         Some(BTree::new_with_children(
//             Section::new((1, 0), (X_LENGTH, Y_LENGTH)),
//             [
//                 // Some(BTree::new(Section::new((20, 0), (X_LENGTH, Y_LENGTH)))),
//                 None,
//                 Some(BTree::new(Section::new((30, 0), (X_LENGTH, Y_LENGTH)))),
//             ],
//         )),
//         Some(BTree::new_with_children(
//             Section::new((4, 0), (X_LENGTH, Y_LENGTH)),
//             [
//                 Some(BTree::new(Section::new((5, 0), (X_LENGTH, Y_LENGTH)))),
//                 None, // BTree::new(Section::new((6, 0), (X_LENGTH, Y_LENGTH))),
//             ],
//         )),
//     ],
// );
//
// // btree.traverse_with_stuff(&mut recursion_count);
// let mut v_btree: Vec<Box<BTree>> = Vec::new();
//
// btree.reach_leaves(&mut v_btree);
//
// prinltn!("{:#?}", v_btree);

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
//     prinltn!("iteration {i}");
//     prinltn!("{}", displayed_map);
//
//     i += 1;
//     thread::sleep(time::Duration::from_millis(1000));
// }
