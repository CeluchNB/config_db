use crate::data_structs::rbt::{RBNode, RBTree};
use crate::data_structs::types::{Key, Value};
use crate::db_constants::{DATA_PATH, DIR_PATH, TABLE_PATH};
use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::marker::PhantomData;
use std::rc::Rc;

pub struct SSTable<K: Key, V: Value> {
    path: String,
    pub offsets: Vec<usize>,
    pub low_key: Vec<u8>,
    pub high_key: Vec<u8>,
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
        tree: &RBTree<K, V>,
    ) -> io::Result<Self> {
        let file_name = format!("{}_l{}_{}.sstable", key_name, level, index);
        let path_name = format!(
            "{}{}/{}{}/{}/{}",
            DIR_PATH, DATA_PATH, db_name, TABLE_PATH, table_name, file_name
        );
        let file = File::create(&path_name)?;
        let mut writer = BufWriter::new(file);

        let mut queue: Vec<SSTableItem<K, V>> = vec![];
        SSTable::get_tree_items(&mut queue, &Some(Rc::clone(&tree.root)));

        let mut offsets: Vec<usize> = Vec::new();
        let mut offset: usize = 0;

        let mut low_key = vec![];
        let mut high_key = vec![];
        let mut start = true;
        for item in queue {
            offsets.push(offset);

            let key_len = item.key.len().to_le_bytes();
            let key = item.key.into_bytes();
            // TODO: this implementation sucks
            if start {
                low_key = key.clone();
                start = false;
            } else {
                high_key = key.clone();
            }
            let val_len = item.val.len().to_le_bytes();
            let val = item.val.into_bytes();

            writer.write_all(&key_len)?;
            writer.write_all(&key)?;
            writer.write_all(&val_len)?;
            writer.write_all(&val)?;
            offset += size_of::<usize>() + key.len() + size_of::<usize>() + val.len();
        }
        writer.flush()?;

        Ok(SSTable {
            path: String::from(path_name),
            offsets: offsets,
            low_key: low_key,
            high_key: high_key,
            phantom_key: PhantomData,
            phantom_val: PhantomData,
        })
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

    // Initialize all tables at start up?
    pub fn read_table(
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

        let mut offsets = Vec::new();
        let mut offset = 0;
        let mut low_key: Vec<u8> = vec![];
        let mut high_key: Vec<u8> = vec![];
        let mut start = true;

        while file.fill_buf()?.len() > 0 {
            offsets.push(offset);
            let mut buf = [0u8; size_of::<usize>()];
            file.read_exact(&mut buf)?;

            let key_len = usize::from_le_bytes(buf);

            // file.seek_relative(key_len as i64)?;
            let mut key_buf = vec![0u8; key_len];
            file.read_exact(&mut key_buf)?;

            if start {
                low_key = key_buf;
                start = false;
            } else {
                high_key = key_buf;
            }

            let mut val_len_buf = [0u8; size_of::<usize>()];
            file.read_exact(&mut val_len_buf)?;

            let val_len = usize::from_le_bytes(val_len_buf);
            file.seek_relative(val_len as i64)?;

            offset += size_of::<usize>() + key_len + size_of::<usize>() + val_len;
        }

        return Ok(Self {
            path: String::from(""),
            offsets: offsets,
            low_key: low_key,
            high_key: high_key,
            phantom_key: PhantomData,
            phantom_val: PhantomData,
        });
    }

    /*fn get_value(&self, key: K) -> std::io::Result<V> {
        Ok(self.phantom_val)
    }*/
}
