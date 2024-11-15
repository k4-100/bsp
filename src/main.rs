use std::{
    // borrow::Borrow,
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    rc::Rc,
    thread,
    time,
};

use rand::{random, Rng};

const X_LENGTH: usize = 140;
const Y_LENGTH: usize = 60;

// const X_LENGTH: usize = 10;
// const Y_LENGTH: usize = 10;
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

#[derive(Debug, Clone)]
enum EntryVariant {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Clone, Debug)]
struct Section {
    pub lt: (usize, usize),
    pub rb: (usize, usize),
    entries: Vec<(usize, usize, EntryVariant)>,
}

impl Section {
    pub fn new(lt: (usize, usize), rb: (usize, usize)) -> Self {
        Self {
            lt,
            rb,
            entries: vec![],
        }
    }

    pub fn contains(&self, point: (usize, usize)) -> bool {
        (self.lt.0 <= point.0 && point.0 < self.rb.0)
            && (self.lt.1 <= point.1 && point.1 < self.rb.1)
    }
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    data: Section,
    left: Option<TreeNodeRef>,
    // left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<TreeNodeRef>,
}

// impl Copy for TreeNode {}

// #[derive(Clone, Copy)]
//
type TreeNodeRef = Rc<RefCell<TreeNode>>;

impl TreeNode {
    pub fn new(data: Section) -> Self {
        Self {
            data,
            left: None,
            right: None,
        }
    }

    pub fn new_with_children(
        data: Section,
        left: Option<TreeNode>,
        right: Option<TreeNode>,
    ) -> Self {
        Self {
            data,
            left: match left {
                Some(left) => Some(Rc::new(RefCell::new(left))),
                None => None,
            },
            right: match right {
                Some(right) => Some(Rc::new(RefCell::new(right))),
                None => None,
            },
        }
    }

    // pub fn reach_leaves(root: TreeNodeRef) -> Vec<TreeNodeRef> {
    //     let mut stack = vec![root];
    //     let mut leaves: Vec<TreeNodeRef> = Vec::new();
    //
    //     while !stack.is_empty() {
    //         let mut has_children = true;
    //         let current: TreeNodeRef = stack.pop().unwrap();
    //
    //         if let Some(left) = &current.borrow().left {
    //             stack.push(left.to_owned());
    //             has_children = false;
    //         };
    //
    //         if let Some(right) = &current.borrow().right {
    //             stack.push(right.to_owned());
    //             has_children = false;
    //         };
    //
    //         if has_children {
    //             leaves.push(current);
    //         }
    //     }
    //
    //     leaves
    // }

    pub fn reach_leaves(root: TreeNodeRef) -> Vec<TreeNodeRef> {
        let mut stack = vec![root];
        let mut leaves: Vec<TreeNodeRef> = Vec::new();

        while !stack.is_empty() {
            let mut has_children = true;
            let current: TreeNodeRef = stack.pop().unwrap();

            if let Some(left) = &current.borrow().left {
                stack.push(left.to_owned());
                has_children = false;
            };

            if let Some(right) = &current.borrow().right {
                stack.push(right.to_owned());
                has_children = false;
            };

            if has_children {
                leaves.push(current);
            }
        }

        leaves
    }

    pub fn split_leaf(leaf: TreeNodeRef) {
        let divide: usize;
        let left_node: Option<TreeNode>;
        let right_node: Option<TreeNode>;
        let mut leaf_borrowed = leaf.borrow_mut();
        let lt = leaf_borrowed.data.lt;
        let rb = leaf_borrowed.data.rb;

        // split with vertical line
        if rand::random() {
            let x_range = lt.0 + 10..rb.0 - 10;
            if x_range.is_empty() {
                return;
            }
            divide = rand::thread_rng().gen_range(x_range);
            left_node = Some(TreeNode::new(Section::new(
                (lt.0, lt.1),
                (divide + 0, rb.1),
            )));
            right_node = Some(TreeNode::new(Section::new(
                (divide + 1, lt.1),
                (rb.0, rb.1),
            )));
        }
        // split with horizontal line
        else {
            let y_range = lt.1 + 5..rb.1 - 5;

            if y_range.is_empty() {
                return;
            }
            divide = rand::thread_rng().gen_range(y_range);
            left_node = Some(TreeNode::new(Section::new(
                (lt.0, lt.1),
                (rb.0, divide + 0),
            )));
            right_node = Some(TreeNode::new(Section::new(
                (lt.0, divide + 1),
                (rb.0, rb.1),
            )));
        }

        // assign new children
        leaf_borrowed.left = Some(Rc::new(RefCell::new(left_node.unwrap())));
        leaf_borrowed.right = Some(Rc::new(RefCell::new(right_node.unwrap())));
    }

    pub fn split_leaves(leaves: Vec<TreeNodeRef>) {
        for leaf in leaves {
            TreeNode::split_leaf(leaf);
        }
    }
}

fn main() {
    let mut tree = TreeNode::new_with_children(
        Section::new((1, 1), (X_LENGTH - 1, Y_LENGTH - 1)),
        None,
        None,
        // Some(TreeNode::new(Section::new(
        //     (2, 1),
        //     (X_LENGTH - 1, Y_LENGTH - 1),
        // ))),
        // Some(TreeNode::new(Section::new(
        //     (3, 1),
        //     (X_LENGTH - 1, Y_LENGTH - 1),
        // ))),
    );
    let mut tree_ref = Rc::new(RefCell::new(tree));
    // let mut leaves: Vec<&Option<TreeNode>> = Vec::new();
    let mut leaves = TreeNode::reach_leaves(tree_ref.clone());
    // TreeNode::split_leaf(leaves[0].clone());

    for _ in 0..10 {
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
                        // displayed_grid[y][x] = "#";
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
