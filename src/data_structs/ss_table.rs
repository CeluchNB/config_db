use crate::data_structs::rbt::{RBNode, RBTree};
use crate::data_structs::types::{Key, Value};
use crate::db_constants::{DATA_PATH, DIR_PATH, TABLE_PATH};
use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::marker::PhantomData;
use std::rc::Rc;

pub struct SSTable<K: Key, V: Value> {
    path: String,
    pub offsets: Vec<usize>,
    pub low_key: Vec<u8>,
    pub high_key: Vec<u8>,
    phantom_key: PhantomData<K>,
    phantom_val: PhantomData<V>,
    db_name: String,
    table_name: String,
    key_name: String,
    level: u8,
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

struct FileItem {
    key_len: usize,
    key_val: Vec<u8>,
    val_len: usize,
    val_val: Vec<u8>,
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
            db_name: String::from(db_name),
            table_name: String::from(table_name),
            key_name: String::from(key_name),
            level: level,
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
            path: path_name,
            offsets: offsets,
            low_key: low_key,
            high_key: high_key,
            phantom_key: PhantomData,
            phantom_val: PhantomData,
            db_name: String::from(db_name),
            table_name: String::from(table_name),
            key_name: String::from(key_name),
            level: level,
        });
    }

    pub fn get_value(&self, key: K) -> Option<V> {
        if key < K::from_utf8(self.low_key.clone()) || key > K::from_utf8(self.high_key.clone()) {
            return None;
        }
        let file = OpenOptions::new().read(true).open(&self.path).unwrap();
        let mut file = BufReader::new(file);

        let offset_len = self.offsets.len();
        let mut idx = offset_len / 2;
        let mut min_idx = 0;
        let mut max_idx = self.offsets.len() - 1;
        loop {
            let current_offset = self.offsets[idx];
            file.seek(SeekFrom::Start(current_offset as u64)).unwrap();

            // read key
            let mut buf = [0u8; size_of::<usize>()];
            file.read_exact(&mut buf).unwrap();

            let key_len = usize::from_le_bytes(buf);

            let mut key_buf = vec![0u8; key_len];
            file.read_exact(&mut key_buf).unwrap();
            let current_key = K::from_utf8(key_buf);
            if current_key == key {
                let mut val_len_buf = [0u8; size_of::<usize>()];
                file.read_exact(&mut val_len_buf).unwrap();

                let val_len = usize::from_le_bytes(val_len_buf);

                let mut val_buf = vec![0u8; val_len];
                file.read_exact(&mut val_buf).unwrap();
                return Some(V::from_utf8(val_buf));
            } else {
                if idx >= max_idx || idx <= min_idx {
                    return None;
                }
                let prev_idx = idx;
                if current_key < key {
                    // halfway between current idx and the end
                    idx = (((idx + max_idx) as f64) / 2_f64).ceil() as usize;
                    min_idx = prev_idx;
                } else {
                    // halfway between current idx and the start
                    idx = (((idx + min_idx) as f64) / 2_f64).floor() as usize;
                    max_idx = prev_idx;
                }
            }
        }
    }

    pub fn merge(&self, other: &SSTable<K, V>) -> io::Result<SSTable<K, V>> {
        let mut self_idx = 0;
        let mut other_idx = 0;

        let file_name = format!("{}_l{}_{}.sstable", self.key_name, self.level + 1, 0);
        let path_name = format!(
            "{}{}/{}{}/{}/{}",
            DIR_PATH, DATA_PATH, self.db_name, TABLE_PATH, self.table_name, file_name
        );
        let new_file = File::create(&path_name)?;
        let mut writer = BufWriter::new(new_file);

        let self_file = OpenOptions::new().read(true).open(&self.path)?;
        let mut self_file: BufReader<File> = BufReader::new(self_file);

        let other_file = OpenOptions::new().read(true).open(&other.path)?;
        let mut other_file: BufReader<File> = BufReader::new(other_file);

        let mut low_key = vec![];
        let mut high_key = vec![];
        let mut offsets = Vec::new();
        let mut offset = 0;
        let mut start = true;

        let mut self_items: Vec<FileItem> = Vec::new();
        loop {
            match SSTable::<K, V>::read_item(&mut self_file) {
                Ok(item) => self_items.push(item),
                Err(_) => break,
            }
        }

        let mut other_items: Vec<FileItem> = Vec::new();
        loop {
            match SSTable::<K, V>::read_item(&mut other_file) {
                Ok(item) => other_items.push(item),
                Err(_) => break,
            }
        }

        loop {
            if self_idx >= self_items.len() && other_idx >= other_items.len() {
                break;
            }
            offsets.push(offset);

            let active_item: &FileItem;
            if self_idx < self_items.len() && other_idx < other_items.len() {
                active_item = if self_items[self_idx].key_val < other_items[other_idx].key_val {
                    self_idx += 1;
                    &self_items[self_idx - 1]
                } else {
                    other_idx += 1;
                    &other_items[other_idx - 1]
                };
            } else if self_idx < self_items.len() {
                active_item = &self_items[self_idx];
                self_idx += 1;
            } else {
                active_item = &other_items[other_idx];
                other_idx += 1;
            }

            if start {
                low_key = active_item.key_val.clone();
                start = false;
            } else {
                high_key = active_item.key_val.clone();
            }

            // write active item
            writer.write_all(&active_item.key_len.to_le_bytes())?;
            writer.write_all(&active_item.key_val)?;
            writer.write_all(&active_item.val_len.to_le_bytes())?;
            writer.write_all(&active_item.val_val)?;

            offset +=
                size_of::<usize>() + active_item.key_len + size_of::<usize>() + active_item.val_len;
        }
        return Ok(SSTable {
            path: String::from(path_name),
            low_key: low_key,
            high_key: high_key,
            offsets: offsets,
            phantom_key: PhantomData,
            phantom_val: PhantomData,
            db_name: String::from(&self.db_name),
            table_name: String::from(&self.table_name),
            key_name: String::from(&self.key_name),
            level: 0,
        });
    }

    fn read_item(self_file: &mut BufReader<File>) -> io::Result<FileItem> {
        let mut buf = [0u8; size_of::<usize>()];
        self_file.read_exact(&mut buf)?;

        let key_len = usize::from_le_bytes(buf);

        let mut key_buf = vec![0u8; key_len];
        self_file.read_exact(&mut key_buf);
        let mut val_len_buf = [0u8; size_of::<usize>()];
        self_file.read_exact(&mut val_len_buf);

        let val_len = usize::from_le_bytes(val_len_buf);

        let mut val_buf = vec![0u8; val_len];
        self_file.read_exact(&mut val_buf);

        return Ok(FileItem {
            key_len: key_len,
            key_val: key_buf,
            val_len: val_len,
            val_val: val_buf,
        });
    }
}
