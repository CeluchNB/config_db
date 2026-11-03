use crate::data_structs::rbt::{RBNode, RBTree};
use crate::data_structs::types::{Key, Value};
use crate::db_constants::{DATA_PATH, DIR_PATH, TABLE_PATH};
use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Write};
use std::marker::PhantomData;
use std::rc::Rc;

pub struct SSTable<K: Key, V: Value> {
    path: String,
    offsets: Vec<usize>,
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
            offsets: vec![],
            phantom_key: PhantomData,
            phantom_val: PhantomData,
        })
    }

    pub fn write_tree(&mut self, tree: &RBTree<K, V>) -> io::Result<()> {
        let file = File::create(&self.path)?;
        let mut writer = BufWriter::new(file);

        let mut queue: Vec<SSTableItem<K, V>> = vec![];
        SSTable::get_tree_items(&mut queue, &Some(Rc::clone(&tree.root)));
        let mut offsets: Vec<usize> = Vec::new();
        let mut offset: usize = 0;
        for item in queue {
            offsets.push(offset);

            let key_len = item.key.len().to_le_bytes();
            let key = item.key.into_bytes();
            let val_len = item.val.len().to_le_bytes();
            let val = item.val.into_bytes();

            writer.write_all(&key_len)?;
            writer.write_all(&key)?;
            writer.write_all(&val_len)?;
            writer.write_all(&val)?;
            offset = size_of::<usize>() + key.len() + size_of::<usize>() + val.len();
        }
        writer.flush()?;
        self.offsets = offsets;
        Ok(())
    }

    fn get_tree_items(res: &mut Vec<SSTableItem<K, V>>, node: &Option<Rc<RefCell<RBNode<K, V>>>>) {
        match node {
            None => return,
            Some(n) => {
                SSTable::get_tree_items(res, &n.borrow().left);
                res.push(SSTableItem::new(
                    n.borrow().key.clone(),
                    n.borrow().val.clone(),
                ));
                SSTable::get_tree_items(res, &n.borrow().right);
            }
        }
    }

    fn read_table(
        db_name: &str,
        table_name: &str,
        key_name: &str,
        level: u8,
        index: u8,
    ) -> io::Result<SSTable<K, V>> {
        let file_name = format!("{}_l{}_{}.sstable", key_name, level, index);
        let path_name = format!(
            "{}{}/{}{}/{}/{}",
            DIR_PATH, DATA_PATH, db_name, TABLE_PATH, table_name, file_name
        );
        let file = OpenOptions::new().read(true).open(&path_name)?;
        let mut file = BufReader::new(file);

        return SSTable::new();
    }

    /*fn get_value(&self, key: K) -> std::io::Result<V> {
        Ok(self.phantom_val)
    }*/
}
