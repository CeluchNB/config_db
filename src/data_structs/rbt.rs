// use std::borrow::Borrow;
use std::cell::RefCell;
use std::fmt::Display;
use std::io;
use std::rc::{Rc, Weak};

#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RebalanceOp {
    Noop,
    Recolor,
    Rotate,
}

pub struct RBNode<K: PartialOrd + Display, V> {
    pub left: Option<Rc<RefCell<RBNode<K, V>>>>,
    pub right: Option<Rc<RefCell<RBNode<K, V>>>>,
    pub parent: Option<Weak<RefCell<RBNode<K, V>>>>,
    pub key: K,
    pub val: V,
    pub color: Color,
}

impl<K: PartialOrd + Display, V> RBNode<K, V> {
    pub fn new(key: K, val: V, color: Color, parent: Option<Weak<RefCell<RBNode<K, V>>>>) -> Self {
        Self {
            left: None,
            right: None,
            parent: parent,
            key: key,
            val: val,
            color: color,
        }
    }
}
// Created "Tree" struct for memory management purposes
// Inserting on the root RBNode that isn't Rc'd made it impossible to clone when setting the parent
// Unsure if there is a better way
pub struct RBTree<K: PartialOrd + Display, V> {
    pub root: Rc<RefCell<RBNode<K, V>>>,
}

impl<K: PartialOrd + Display, V> RBTree<K, V> {
    pub fn new(root: Rc<RefCell<RBNode<K, V>>>) -> Self {
        Self { root: root }
    }

    pub fn insert(&self, key: K, val: V) -> Result<(), io::Error> {
        let node = Self::initial_insert(&self.root, key, val);

        // if parent is black, we can return
        let child = Rc::clone(&node);
        // not safely unwrapping b/c tree is initialized with root
        let parent: Weak<RefCell<RBNode<K, V>>> = child.borrow().parent.unwrap();
        if parent.upgrade().unwrap().borrow().color == Color::Black {
            return Ok(());
        }

        let rebalance_op = Self::get_rebalance_op(&parent);
        if rebalance_op == RebalanceOp::Recolor {
            // recursively recolor up the tree and be done
            Self::recolor(&node);
            return Ok(());
        }

        if rebalance_op == RebalanceOp::Rotate {
            Self::rotate(&node);
        }
        // uncle is black, balance by rotation
        Ok(())
    }

    fn initial_insert(
        root: &Rc<RefCell<RBNode<K, V>>>,
        key: K,
        val: V,
    ) -> Rc<RefCell<RBNode<K, V>>> {
        let mut current_node: Rc<RefCell<RBNode<K, V>>> = Rc::clone(root);
        loop {
            let next = {
                let node = current_node.borrow();
                if node.key < key {
                    match &node.right {
                        Some(right) => Some(Rc::clone(right)),
                        None => None,
                    }
                } else {
                    match &node.left {
                        Some(left) => Some(Rc::clone(left)),
                        None => None,
                    }
                }
            };

            match next {
                Some(child) => current_node = child,
                None => {
                    let current_cloned = Rc::clone(&current_node);
                    if key < current_cloned.borrow().key {
                        let ref_cell = RefCell::new(RBNode::new(
                            key,
                            val,
                            Color::Red,
                            Some(Rc::downgrade(&current_node)),
                        ));
                        let new_node = Rc::new(ref_cell);
                        current_cloned.borrow_mut().left = Some(Rc::clone(&new_node));
                        return new_node;
                    } else {
                        let ref_cell = RefCell::new(RBNode::new(
                            key,
                            val,
                            Color::Red,
                            Some(Rc::downgrade(&current_node)),
                        ));
                        let new_node = Rc::new(ref_cell);
                        current_cloned.borrow_mut().right = Some(Rc::clone(&new_node));
                        return new_node;
                    }
                }
            }
        }
    }

    fn get_rebalance_op(parent: &Weak<RefCell<RBNode<K, V>>>) -> RebalanceOp {
        let grand_parent;
        match &parent.upgrade().unwrap().as_ref().borrow().parent {
            Some(gp) => grand_parent = gp.clone(),
            None => return RebalanceOp::Noop,
        }

        let uncle;
        if parent.upgrade().unwrap().borrow().key > grand_parent.upgrade().unwrap().borrow().key {
            match &grand_parent.upgrade().unwrap().borrow().left {
                Some(u) => uncle = Rc::clone(u),
                // uncle is black if it is None
                None => return RebalanceOp::Rotate,
            }
        } else {
            match &grand_parent.upgrade().unwrap().borrow().right {
                Some(u) => uncle = Rc::clone(u),
                // uncle is black if it is None
                None => return RebalanceOp::Rotate,
            }
        }

        if uncle.borrow().color == Color::Red {
            return RebalanceOp::Recolor;
        } else {
            return RebalanceOp::Rotate;
        }
    }

    fn recolor(child: &Rc<RefCell<RBNode<K, V>>>) -> Rc<RefCell<RBNode<K, V>>> {
        let mut node = Rc::clone(child);
        loop {
            let parent = Rc::clone(node.borrow().parent.as_ref().unwrap());
            // If parent is black, we do not recolor
            if parent.borrow().color == Color::Black {
                return node;
            }

            let grand_parent = Rc::clone(parent.borrow().parent.as_ref().unwrap());
            let uncle;
            // if the parent's key is greater than the grandparent's key, the parent is the right
            // node, so the uncle is the left node
            if parent.borrow().key > grand_parent.borrow().key {
                match grand_parent.borrow().left.as_ref() {
                    Some(u) => {
                        uncle = Rc::clone(&u);
                    }
                    None => {
                        // uncle is black, move on
                        return node;
                    }
                }
            } else {
                match grand_parent.borrow().right.as_ref() {
                    Some(u) => {
                        uncle = Rc::clone(&u);
                    }
                    None => {
                        // uncle is black, move on
                        return node;
                    }
                }
            }

            // if parent + uncle are red, make them both black and make grandparent red
            if parent.borrow().color == Color::Red && uncle.borrow().color == Color::Red {
                parent.borrow_mut().color = Color::Black;
                uncle.borrow_mut().color = Color::Black;
                if grand_parent.borrow().parent.is_none() {
                    // we reached the root, everything is good
                    return grand_parent;
                }
                grand_parent.borrow_mut().color = Color::Red;
            }

            node = Rc::clone(&grand_parent);
        }
    }

    fn rotate(node: &Rc<RefCell<RBNode<K, V>>>) {
        let parent = Rc::clone(node.borrow().parent.as_ref().unwrap());
        let grand_parent = Rc::clone(parent.borrow().parent.as_ref().unwrap());
        if grand_parent.borrow().key > node.borrow().key {
            Self::right_rotate(node);
        } else {
            Self::left_rotate(node);
        }
    }

    fn right_rotate(node: &Rc<RefCell<RBNode<K, V>>>) {
        let parent = Rc::clone(node.borrow().parent.as_ref().unwrap()); // 1
        let grand_parent = Rc::clone(parent.borrow().parent.as_ref().unwrap()); // 5
        let great_grand_parent;
        match &grand_parent.borrow().parent {
            Some(ggp) => great_grand_parent = Some(Rc::clone(ggp)),
            None => great_grand_parent = None,
        }

        let final_parent;
        if node.borrow().key > parent.borrow().key {
            Self::right_inner_to_left_outer(node);
            final_parent = Rc::clone(&node);
        } else {
            final_parent = Rc::clone(&parent);
        }

        match great_grand_parent {
            Some(ggp) => {
                ggp.borrow_mut().left = Some(Rc::clone(&final_parent));
                final_parent.borrow_mut().parent = Some(ggp);
            }
            None => {
                final_parent.borrow_mut().parent = None;
            }
        }

        if let Some(initial_right) = parent.borrow().right.as_ref() {
            grand_parent.borrow_mut().left = Some(Rc::clone(initial_right));
        }
        final_parent.borrow_mut().right = Some(Rc::clone(&grand_parent));
        grand_parent.borrow_mut().parent = Some(Rc::clone(&final_parent));
        grand_parent.borrow_mut().left = None;

        grand_parent.borrow_mut().color = Color::Red;
        final_parent.borrow_mut().color = Color::Black;
    }

    fn left_rotate(node: &Rc<RefCell<RBNode<K, V>>>) {
        let parent = Rc::clone(node.borrow().parent.as_ref().unwrap());
        let grand_parent = Rc::clone(parent.borrow().parent.as_ref().unwrap());
        let great_grand_parent;
        match &grand_parent.borrow().parent {
            Some(ggp) => great_grand_parent = Some(Rc::clone(ggp)),
            None => great_grand_parent = None,
        }

        let final_parent;
        if node.borrow().key < parent.borrow().key {
            Self::left_inner_to_right_outer(node);
            final_parent = Rc::clone(&node);
        } else {
            final_parent = Rc::clone(&parent);
        }

        match great_grand_parent {
            Some(ggp) => {
                ggp.borrow_mut().right = Some(Rc::clone(&final_parent));
                final_parent.borrow_mut().parent = Some(ggp);
            }
            None => {
                final_parent.borrow_mut().parent = None; // Hit
            }
        }

        if let Some(initial_left) = final_parent.borrow().left.as_ref() {
            grand_parent.borrow_mut().right = Some(Rc::clone(initial_left));
        }
        final_parent.borrow_mut().left = Some(Rc::clone(&grand_parent));
        grand_parent.borrow_mut().parent = Some(Rc::clone(&final_parent));
        grand_parent.borrow_mut().right = None;

        grand_parent.borrow_mut().color = Color::Red;
        final_parent.borrow_mut().color = Color::Black;
    }

    fn right_inner_to_left_outer(node: &Rc<RefCell<RBNode<K, V>>>) {
        let parent = Rc::clone(node.borrow().parent.as_ref().unwrap());
        let grand_parent = Rc::clone(parent.borrow().parent.as_ref().unwrap());

        grand_parent.borrow_mut().left = Some(Rc::clone(node));
        parent.borrow_mut().right = None;
        parent.borrow_mut().parent = Some(Rc::clone(node));
        node.borrow_mut().left = Some(parent);
        node.borrow_mut().parent = Some(grand_parent);
    }

    fn left_inner_to_right_outer(node: &Rc<RefCell<RBNode<K, V>>>) {
        let parent = Rc::clone(node.borrow().parent.as_ref().unwrap());
        let grand_parent = Rc::clone(parent.borrow().parent.as_ref().unwrap());

        grand_parent.borrow_mut().right = Some(Rc::clone(node));
        parent.borrow_mut().left = None;
        parent.borrow_mut().parent = Some(Rc::clone(node));
        node.borrow_mut().right = Some(parent);
    }

    pub fn find(&self, key: K) -> Option<Rc<RefCell<RBNode<K, V>>>> {
        let mut current_node = Rc::clone(&self.root);

        loop {
            let next = {
                let cloned_current = Rc::clone(&current_node);
                let node = cloned_current.borrow();
                if node.key < key {
                    match &node.right {
                        Some(right) => Some(Rc::clone(right)),
                        None => None,
                    }
                } else if node.key > key {
                    match &node.left {
                        Some(left) => Some(Rc::clone(left)),
                        None => None,
                    }
                } else {
                    return Some(Rc::clone(&current_node));
                }
            };

            match next {
                Some(child) => {
                    current_node = child;
                }
                None => {
                    return None;
                }
            }
        }
    }
}
