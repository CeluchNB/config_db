// use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::fmt::Display;
use std::io;
use std::rc::{Rc, Weak};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
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

    pub fn print_me(&self) {
        let mut str_builder = String::new();
        str_builder
            .push_str(format!("Node -> Key: {}, Color: {:?}", self.key, self.color).as_str());
        if let Some(p) = &self.parent {
            str_builder
                .push_str(format!(", Parent Key: {}", p.upgrade().unwrap().borrow().key).as_str());
        } else {
            str_builder.push_str(", IS ROOT");
        }

        if let Some(l) = &self.left {
            str_builder.push_str(format!(", Left Key: {}", l.borrow().key).as_str());
        }

        if let Some(r) = &self.right {
            str_builder.push_str(format!(", Right Key: {}", r.borrow().key).as_str());
        }
        println!("{}", str_builder);
    }
}
pub struct RBTree<K: PartialOrd + Display, V> {
    pub root: Rc<RefCell<RBNode<K, V>>>,
}

impl<K: PartialOrd + Display, V> RBTree<K, V> {
    pub fn new(root: Rc<RefCell<RBNode<K, V>>>) -> Self {
        Self { root: root }
    }

    pub fn insert(&self, key: K, val: V) -> Result<(), io::Error> {
        println!("Inserting {}", key);
        let node = Self::initial_insert(&self.root, key, val);

        Self::rebalance(&node)
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

    fn rebalance(node: &Rc<RefCell<RBNode<K, V>>>) -> Result<(), io::Error> {
        let mut current_node = Rc::clone(node);
        loop {
            let parent = current_node.borrow().parent.clone();
            match parent {
                Some(p) => {
                    if p.upgrade().unwrap().borrow().color == Color::Black {
                        return Ok(());
                    }
                }
                None => {
                    // set root to black
                    current_node.borrow_mut().color = Color::Black;
                    return Ok(());
                }
            }

            current_node = Self::recolor(&current_node);
            Self::rotate(&current_node);
        }
    }

    fn recolor(child: &Rc<RefCell<RBNode<K, V>>>) -> Rc<RefCell<RBNode<K, V>>> {
        let mut node = Rc::clone(child);
        loop {
            let parent = node.borrow().parent.as_ref().unwrap().upgrade().unwrap();
            // If parent is black, we do not recolor
            if parent.borrow().color == Color::Black {
                return node;
            }

            let grand_parent;
            match parent.borrow().parent.as_ref() {
                Some(gp) => grand_parent = gp.upgrade().unwrap(),
                None => return node,
            }
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
        let right_rotate: bool;
        {
            let parent;
            match node.borrow().parent.as_ref() {
                Some(p) => parent = p.upgrade().unwrap(),
                None => return,
            }
            let grand_parent;
            match parent.borrow().parent.as_ref() {
                Some(gp) => grand_parent = gp.upgrade().unwrap(),
                None => return,
            }
            let uncle_color = {
                if parent.borrow().key < grand_parent.borrow().key {
                    match grand_parent.borrow().right.as_ref() {
                        Some(u) => u.borrow().color,
                        None => Color::Black,
                    }
                } else {
                    match grand_parent.borrow().left.as_ref() {
                        Some(u) => u.borrow().color,
                        None => Color::Black,
                    }
                }
            };
            println!("Uncle color: {:?}", uncle_color);

            if uncle_color == Color::Red {
                return;
            }

            right_rotate = grand_parent.borrow().key > node.borrow().key;
        }
        if right_rotate {
            println!("Rotating right");
            Self::right_rotate(node);
        } else {
            println!("Rotating left");
            Self::left_rotate(node);
        }
    }

    fn right_rotate(node: &Rc<RefCell<RBNode<K, V>>>) {
        let parent = node.borrow().parent.as_ref().unwrap().upgrade().unwrap(); // 1
        let grand_parent = parent.borrow().parent.as_ref().unwrap().upgrade().unwrap(); // 5
        let great_grand_parent;
        match &grand_parent.borrow().parent {
            Some(ggp) => great_grand_parent = Some(ggp.upgrade().unwrap()),
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
                final_parent.borrow_mut().parent = Some(Rc::downgrade(&ggp));
            }
            None => {
                final_parent.borrow_mut().parent = None;
            }
        }

        if let Some(initial_right) = parent.borrow().right.as_ref() {
            grand_parent.borrow_mut().left = Some(Rc::clone(initial_right));
        }
        final_parent.borrow_mut().right = Some(Rc::clone(&grand_parent));
        grand_parent.borrow_mut().parent = Some(Rc::downgrade(&final_parent));
        grand_parent.borrow_mut().left = None; // TODO: Preserve subtree?

        grand_parent.borrow_mut().color = Color::Red;
        final_parent.borrow_mut().color = Color::Black;
    }

    fn left_rotate(node: &Rc<RefCell<RBNode<K, V>>>) {
        let parent = node.borrow().parent.as_ref().unwrap().upgrade().unwrap();
        let grand_parent = parent.borrow().parent.as_ref().unwrap().upgrade().unwrap();
        let great_grand_parent;
        match &grand_parent.borrow().parent {
            Some(ggp) => great_grand_parent = Some(ggp.upgrade().unwrap()),
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
                if ggp.borrow().key < final_parent.borrow().key {
                    ggp.borrow_mut().right = Some(Rc::clone(&final_parent));
                } else {
                    ggp.borrow_mut().left = Some(Rc::clone(&final_parent));
                }
                final_parent.borrow_mut().parent = Some(Rc::downgrade(&ggp));
            }
            None => {
                final_parent.borrow_mut().parent = None; // Hit
            }
        }

        if let Some(initial_left) = final_parent.borrow().left.as_ref() {
            grand_parent.borrow_mut().right = Some(Rc::clone(initial_left));
            initial_left.borrow_mut().parent = Some(Rc::downgrade(&grand_parent));
        }
        final_parent.borrow_mut().left = Some(Rc::clone(&grand_parent));
        grand_parent.borrow_mut().parent = Some(Rc::downgrade(&final_parent));
        grand_parent.borrow_mut().right = None;

        grand_parent.borrow_mut().color = Color::Red;
        final_parent.borrow_mut().color = Color::Black;
    }

    fn right_inner_to_left_outer(node: &Rc<RefCell<RBNode<K, V>>>) {
        let parent = node.borrow().parent.as_ref().unwrap().upgrade().unwrap();
        let grand_parent = parent.borrow().parent.as_ref().unwrap().upgrade().unwrap();

        grand_parent.borrow_mut().left = Some(Rc::clone(node)); // TODO: START HERE - Can I create a
        // new clone without causing a
        // memory leak?
        parent.borrow_mut().right = None;
        parent.borrow_mut().parent = Some(Rc::downgrade(&node));
        node.borrow_mut().left = Some(Rc::clone(&parent));
        node.borrow_mut().parent = Some(Rc::downgrade(&grand_parent));
    }

    fn left_inner_to_right_outer(node: &Rc<RefCell<RBNode<K, V>>>) {
        let parent = node.borrow().parent.as_ref().unwrap().upgrade().unwrap();
        let grand_parent = parent.borrow().parent.as_ref().unwrap().upgrade().unwrap();

        grand_parent.borrow_mut().right = Some(Rc::clone(node));
        parent.borrow_mut().left = None;
        parent.borrow_mut().parent = Some(Rc::downgrade(&node));
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
