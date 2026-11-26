use crate::data_structs::{RBTree, SSTable};
use crate::db_constants::{DATA_PATH, DIR_PATH};
use std::collections::HashMap;
use std::fs;
use std::io;

pub struct SSTableManager {
    db_name: String,
    table_name: String,
    key: String,
    tables: HashMap<u8, Vec<SSTable<String, String>>>,
}

impl SSTableManager {
    pub fn new(db_name: &str, table_name: &str, key: &str) -> Self {
        return Self {
            db_name: String::from(db_name),
            table_name: String::from(table_name),
            key: String::from(key),
            tables: HashMap::new(),
        };
    }

    // atomic write commercial
    pub fn write(&self, tree: &RBTree<String, String>) -> io::Result<()> {
        // find correct table
        // insert into table
        // decide whether to merge
        // merge to next level
        // always writing to a l0 table
        // recursively decide to merge
        // TODO: START HERE
        let table = Some(
            SSTable::new(
                self.db_name.as_str(),
                self.table_name.as_str(),
                self.key.as_str(),
                0,
                0,
                tree,
            )
            .unwrap(),
        );

        Ok(())
    }

    fn all_tables(&self) -> io::Result<Vec<String>> {
        let table_dir = format!(
            "{}{}/{}{}/{}",
            DIR_PATH, DATA_PATH, self.db_name, "/tables", self.table_name
        );

        let mut files = vec![];
        for entry in fs::read_dir(table_dir)? {
            let entry = entry?;
            let file_type = entry.file_type()?;

            if file_type.is_file() {
                if entry
                    .file_name()
                    .to_string_lossy()
                    .contains(self.key.as_str())
                {
                    files.push(entry.file_name().to_string_lossy().to_string());
                }
            }
        }

        return Ok(files);
    }

    pub fn should_merge() -> bool {
        return true;
    }

    pub fn next_index(files: Vec<String>) -> u8 {
        let mut index = 0;
        for file in files {}

        return 8;
    }

    pub fn next_level() -> String {
        return String::from("");
    }

    fn parse_file(file: &str) -> [u8; 2] {
        let elements: Vec<&str> = file.split("_").collect();
        return [
            elements[1][1..].parse().unwrap(),
            elements[2].parse().unwrap(),
        ];
    }
}
