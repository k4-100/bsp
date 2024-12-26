use std::{cell::RefCell, rc::Rc};

use rand::Rng;

pub const X_LENGTH: usize = 180;
pub const Y_LENGTH: usize = 60;

// #region Section

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryVariant {
    Top,
    Right,
    Bottom,
    Left,
    NotSet,
}

#[derive(Clone, Debug)]
pub struct Section {
    pub lt: (usize, usize),
    pub rb: (usize, usize),
    passages: Vec<EntryVariant>,
}

impl Section {
    pub fn new(lt: (usize, usize), rb: (usize, usize)) -> Self {
        Self {
            lt,
            rb,
            passages: vec![],
        }
    }
    pub fn new_with_random_passages(lt: (usize, usize), rb: (usize, usize)) -> Self {
        let passages = vec![
            EntryVariant::Top,    //    ( (lt.0 + rb.0/2), (lt.1) )
            EntryVariant::Right,  //    ( (rb.0), (lt.1 + rb.1/2) )
            EntryVariant::Bottom, //    ( (lt.0 + rb.0/2), (br.1) )
            EntryVariant::Left,   //    ( (lt.0), (lt.1 + rb.1/2) )
        ];
        Self { lt, rb, passages }
    }

    pub fn contains(&self, point: (usize, usize)) -> bool {
        (self.lt.0 <= point.0 && point.0 < self.rb.0)
            && (self.lt.1 <= point.1 && point.1 < self.rb.1)
    }
    pub fn check_if_is_passage(
        &self,
        point: (usize, usize),
    ) -> Option<((usize, usize), EntryVariant)> {
        let Section { lt, rb, passages } = self;

        // println!("x: {:#?} y: {:#?}", lt.0 + rb.0 / 2, lt.1);
        // println!("contains: {}", passages.contains(&EntryVariant::Top));
        // if passages.contains(&EntryVariant::Top) && point == (lt.0 + (rb.0 / 2), lt.1) {
        // if passages.contains(&EntryVariant::Top) &&
        // for passage in self.passages.iter() {
        //     match passage {
        //         EntryVariant::Top => {
        //             if point == (lt.0 + ((rb.0 - lt.0) / 2), lt.1) {
        //                 return Some((point, EntryVariant::Top));
        //             }
        //         }
        //         EntryVariant::Right => {
        //             if point == ((rb.0) - 1, (lt.1 + rb.1 / 2)) {
        //                 return Some((point, EntryVariant::Right));
        //             }
        //         }
        //         EntryVariant::Bottom => {
        //             if point == ((lt.0 + rb.0 / 2), rb.1 - 1) {
        //                 return Some((point, EntryVariant::Bottom));
        //             }
        //         }
        //         EntryVariant::Left => {
        //             if point == ((lt.0), (lt.1 + rb.1 / 2)) {
        //                 return Some((point, EntryVariant::Left));
        //             }
        //         }
        //         _ => {}
        //     }
        // }

        if point == (lt.0 + ((rb.0 - lt.0) / 2), lt.1) {
            return Some((point, EntryVariant::Top));
        }

        if point == ((rb.0) - 1, (lt.1 + rb.1 / 2)) {
            return Some((point, EntryVariant::Right));
        }
        if point == ((lt.0 + rb.0 / 2), rb.1 - 1) {
            return Some((point, EntryVariant::Bottom));
        }
        if point == ((lt.0), (lt.1 + rb.1 / 2)) {
            return Some((point, EntryVariant::Left));
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub data: Section,
    pub left: Option<TreeNodeRef>,
    // left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<TreeNodeRef>,
}

//#endregion !Section

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
            left_node = Some(TreeNode::new(Section::new_with_random_passages(
                (lt.0, lt.1),
                (divide + 0, rb.1),
            )));
            right_node = Some(TreeNode::new(Section::new_with_random_passages(
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
            left_node = Some(TreeNode::new(Section::new_with_random_passages(
                (lt.0, lt.1),
                (rb.0, divide + 0),
            )));
            right_node = Some(TreeNode::new(Section::new_with_random_passages(
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

pub fn generate_map() -> Vec<Vec<&'static str>> {
    let tree = TreeNode::new_with_children(
        Section::new_with_random_passages((1, 1), (X_LENGTH - 1, Y_LENGTH - 1)),
        None,
        None,
    );
    let tree_ref = Rc::new(RefCell::new(tree));
    let mut leaves = TreeNode::reach_leaves(tree_ref.clone());

    for _ in 0..7 {
        TreeNode::split_leaves(leaves.clone());
        leaves = TreeNode::reach_leaves(tree_ref.clone());
    }

    let mut displayed_grid: Vec<Vec<&str>> = (0..Y_LENGTH)
        .map(|_| (0..X_LENGTH).map(|_| "#").collect::<Vec<&str>>())
        .collect();

    let mut passages: Vec<((usize, usize), EntryVariant)> = Vec::new();

    if !leaves.is_empty() {
        for leaf in leaves.iter().rev() {
            let leaf_unwrapped = &leaf.borrow();
            // let Section { lt, rb, .. } = leaf_unwrapped.data;
            // let fuck: (usize, usize) = (lt.0 + ((rb.0 - lt.0) / 2), lt.1);

            // println!("TOP: {:?}", fuck);
            // println!("lt: {:?} rb: {:?}", lt, rb);

            for y in 0..Y_LENGTH {
                for x in 0..X_LENGTH {
                    if leaf_unwrapped.data.contains((x, y)) {
                        displayed_grid[y][x] = ".";
                    }

                    // println!("x: {x} : y: {y}");
                    if let Some(stored_passage) =
                        passages.iter().find(|s_p| s_p.0 .0 == x && s_p.0 .1 == y)
                    {
                        match stored_passage.1 {
                            EntryVariant::Top => displayed_grid[y - 1][x] = ".",
                            EntryVariant::Left => displayed_grid[y][x - 1] = ".",
                            EntryVariant::Bottom => displayed_grid[y + 1][x] = ".",
                            EntryVariant::Right => displayed_grid[y][x + 1] = ".",
                            _ => {}
                        }
                    } else if let Some(new_passage) =
                        leaf_unwrapped.data.check_if_is_passage((x, y))
                    {
                        match new_passage.1 {
                            EntryVariant::Top => displayed_grid[y - 1][x] = ".",
                            EntryVariant::Left => displayed_grid[y][x - 1] = ".",
                            EntryVariant::Bottom => displayed_grid[y + 1][x] = ".",
                            EntryVariant::Right => displayed_grid[y][x + 1] = ".",
                            _ => {}
                        }
                        passages.push(new_passage);
                    }
                }
            }
        }
    }

    displayed_grid
}
