use crate::data_structs::rbt::{RBNode, RBTree};
use crate::data_structs::types::{Key, Value};
use crate::db_constants::{DATA_PATH, DIR_PATH, TABLE_PATH};
use std::cell::RefCell;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::marker::PhantomData;
use std::rc::Rc;

pub struct SSTable<K: Key, V: Value> {
    path: String,
    phantom_key: PhantomData<K>,
    phantom_val: PhantomData<V>,
}

struct SSTableItem<K: Key, V: Value> {
    key: K,
    val: V,
}

impl<K: Key, V: Value> SSTableItem<K, V> {
    fn new(key: K, val: V) -> Self {
        Self { key, val }
    }
}

impl<K: Key, V: Value> SSTable<K, V> {
    pub fn new(
        db_name: &str,
        table_name: &str,
        key_name: &str,
        level: u8,
        index: u8,
    ) -> io::Result<Self> {
        println!("SS TABLE NEW");
        let file_name = format!("{}_l{}_{}.sstable", key_name, level, index);
        let path_name = format!(
            "{}{}/{}{}/{}/{}",
            DIR_PATH, DATA_PATH, db_name, TABLE_PATH, table_name, file_name
        );
        Ok(SSTable {
            path: String::from(path_name),
            phantom_key: PhantomData,
            phantom_val: PhantomData,
        })
    }

    pub fn write_tree(&self, tree: &RBTree<K, V>) -> io::Result<()> {
        println!("SSTABLE WRITE TREE");
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.path)?;

        let mut queue: Vec<SSTableItem<K, V>> = vec![];
        println!("SSTABLE BEFORE GET ITEMS");
        SSTable::get_tree_items(&mut queue, &Some(Rc::clone(&tree.root)));
        for item in queue {
            println!("WRITING {} {}", item.key, item.val);
            file.write_all(&item.key.len().to_le_bytes())?;
            file.write_all(&item.key.into_bytes())?;
            file.write_all(&item.val.len().to_le_bytes())?;
            file.write_all(&item.val.into_bytes())?;
        }
        file.flush()?;
        Ok(())
    }

    fn get_tree_items(res: &mut Vec<SSTableItem<K, V>>, node: &Option<Rc<RefCell<RBNode<K, V>>>>) {
        println!("SSTABLE GET TREE ITEMS");
        match node {
            None => return,
            Some(n) => {
                println!("GET TREE ITEMS SOME {}", n.borrow().key);
                SSTable::get_tree_items(res, &n.borrow().left);
                res.push(SSTableItem::new(
                    n.borrow().key.clone(),
                    n.borrow().val.clone(),
                ));
                SSTable::get_tree_items(res, &n.borrow().right);
            }
        }
    }

    /*fn get_value(&self, key: K) -> std::io::Result<V> {
        Ok(self.phantom_val)
    }*/
}
