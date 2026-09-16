// use std::borrow::Borrow;
use std::cell::RefCell;
use std::io;
use std::rc::Rc;

pub struct RBNode<K: PartialOrd, V> {
    pub left: Option<Rc<RefCell<RBNode<K, V>>>>,
    pub right: Option<Rc<RefCell<RBNode<K, V>>>>,
    pub parent: Option<Rc<RefCell<RBNode<K, V>>>>,
    pub key: K,
    pub val: V,
    pub red: bool,
}

impl<K: PartialOrd, V> RBNode<K, V> {
    pub fn new(key: K, val: V, parent: Option<Rc<RefCell<RBNode<K, V>>>>) -> Self {
        Self {
            left: None,
            right: None,
            parent: parent,
            key: key,
            val: val,
            red: false,
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
        // insert
        let mut current_node: Rc<RefCell<RBNode<K, V>>> = Rc::clone(&self.root);
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
                        let ref_cell = RefCell::new(RBNode::new(key, val, Some(current_node)));
                        let new_node = Rc::new(ref_cell);
                        current_cloned.borrow_mut().left = Some(new_node);
                    } else {
                        let ref_cell = RefCell::new(RBNode::new(key, val, Some(current_node)));
                        let new_node = Rc::new(ref_cell);
                        current_cloned.borrow_mut().right = Some(new_node);
                    }
                    break;
                }
            }
        }

        // balance
        Ok(())
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
                    println!("Find setting current node");
                    current_node = child;
                }
                None => {
                    println!("Find failed");
                    return None;
                }
            }
        }
    }
}
