use std::io;

pub struct RBNode<K: PartialOrd, V> {
    pub left: Option<Box<RBNode<K, V>>>,
    pub right: Option<Box<RBNode<K, V>>>,
    pub key: K,
    pub val: V,
    pub red: bool,
}

impl<K: PartialOrd, V> RBNode<K, V> {
    pub fn new(key: K, val: V) -> Self {
        Self {
            left: None,
            right: None,
            key: key,
            val: val,
            red: false,
        }
    }

    pub fn insert(&mut self, key: K, val: V) -> Result<(), io::Error> {
        // insert
        let mut current_node = self;
        loop {
            if current_node.key < key {
                if let Some(ref mut child) = current_node.right {
                    current_node = child;
                } else {
                    // let new_node = RBNode::new(key, val);
                    current_node.right = Some(Box::new(RBNode::new(key, val)));
                    break;
                }
            } else if current_node.key > key {
                if let Some(ref mut child) = current_node.left {
                    current_node = child;
                } else {
                    current_node.left = Some(Box::new(RBNode::new(key, val)));
                    break;
                }
            }
        }

        // balance
        Ok(())
    }

    pub fn find(self, key: K) -> Option<RBNode<K, V>> {
        if key == self.key {
            return Some(self);
        } else if key < self.key {
            let left = self.left.unwrap();
            return left.find(key);
        } else if key > self.key {
            let right = self.right.unwrap();
            return right.find(key);
        }

        return None;
    }
}
