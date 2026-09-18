// use std::borrow::Borrow;
use std::cell::RefCell;
use std::io;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

pub struct RBNode<K: PartialOrd, V> {
    pub left: Option<Rc<RefCell<RBNode<K, V>>>>,
    pub right: Option<Rc<RefCell<RBNode<K, V>>>>,
    pub parent: Option<Rc<RefCell<RBNode<K, V>>>>,
    pub key: K,
    pub val: V,
    pub color: Color,
}

impl<K: PartialOrd, V> RBNode<K, V> {
    pub fn new(key: K, val: V, color: Color, parent: Option<Rc<RefCell<RBNode<K, V>>>>) -> Self {
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
pub struct RBTree<K: PartialOrd, V> {
    pub root: Rc<RefCell<RBNode<K, V>>>,
}

impl<K: PartialOrd, V> RBTree<K, V> {
    pub fn new(root: Rc<RefCell<RBNode<K, V>>>) -> Self {
        Self { root: root }
    }

    pub fn insert(&self, key: K, val: V) -> Result<(), io::Error> {
        // TODO: does this node return a correct value?
        let node = Self::initial_insert(&self.root, key, val);

        let _starting_node = Self::recolor(node);
        // balance
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
                        let ref_cell =
                            RefCell::new(RBNode::new(key, val, Color::Red, Some(current_node)));
                        let new_node = Rc::new(ref_cell);
                        current_cloned.borrow_mut().left = Some(Rc::clone(&new_node));
                        return new_node;
                    } else {
                        let ref_cell =
                            RefCell::new(RBNode::new(key, val, Color::Red, Some(current_node)));
                        let new_node = Rc::new(ref_cell);
                        current_cloned.borrow_mut().right = Some(Rc::clone(&new_node));
                        return new_node;
                    }
                }
            }
        }
    }

    fn recolor(child: Rc<RefCell<RBNode<K, V>>>) -> Rc<RefCell<RBNode<K, V>>> {
        let mut node = Rc::clone(&child);
        loop {
            let parent = Rc::clone(&node.borrow().parent.as_ref().unwrap());
            // If parent is black, we do not recolor
            if parent.borrow().color == Color::Black {
                return node;
            }

            let grand_parent = Rc::clone(&parent.borrow().parent.as_ref().unwrap());
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

    fn right_rotate(child: Rc<RefCell<RBNode<K, V>>>) {}

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
