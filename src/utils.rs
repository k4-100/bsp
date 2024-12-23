use std::{
    // borrow::Borrow,
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    rc::Rc,
    thread,
    time,
};

use rand::{random, Rng};

pub const X_LENGTH: usize = 140;
pub const Y_LENGTH: usize = 60;

#[derive(Debug, Clone)]
pub enum EntryVariant {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Clone, Debug)]
pub struct Section {
    pub lt: (usize, usize),
    pub rb: (usize, usize),
    passages: Vec<(usize, usize, EntryVariant)>,
}

impl Section {
    pub fn new(lt: (usize, usize), rb: (usize, usize)) -> Self {
        Self {
            lt,
            rb,
            passages: vec![],
        }
    }
    pub fn new_with_random_passages(
        lt: (usize, usize),
        rb: (usize, usize),
        passages: Vec<(usize, usize, EntryVariant)>,
    ) -> Self {
        Self { lt, rb, passages }
    }

    pub fn contains(&self, point: (usize, usize)) -> bool {
        (self.lt.0 <= point.0 && point.0 < self.rb.0)
            && (self.lt.1 <= point.1 && point.1 < self.rb.1)
    }
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub data: Section,
    pub left: Option<TreeNodeRef>,
    // left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<TreeNodeRef>,
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
