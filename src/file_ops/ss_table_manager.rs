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
        let mut map: HashMap<u8, Vec<SSTable<String, String>>> = HashMap::new();
        map.insert(0, Vec::new());
        return Self {
            db_name: String::from(db_name),
            table_name: String::from(table_name),
            key: String::from(key),
            tables: map,
        };
    }

    // atomic write commercial
    pub fn write(&mut self, tree: &RBTree<String, String>) -> io::Result<()> {
        let mut level: u8 = 0;
        while let Some(table_vec) = self.tables.get_mut(&level) {
            let index = table_vec.len();
            // TODO: START HERE - Getting to the 8th element fails
            if level == 0 {
                // actually insert the tree
                println!("INSERTING INTO NEW TREE");
                let table = SSTable::new(
                    self.db_name.as_str(),
                    self.table_name.as_str(),
                    self.key.as_str(),
                    level,
                    index as u8,
                    tree,
                )
                .unwrap();

                table_vec.push(table);
            }

            level += 1;
            if table_vec.len() > 1 {
                // merge if more than 2 trees in level
                let table1 = &table_vec[0];
                let table2 = &table_vec[1];
                println!("MERGING");

                let new_table = table1.merge(table2)?;
                for ss_table in &mut *table_vec {
                    println!("DELETING VEC");
                    ss_table.delete()?;
                }
                table_vec.clear();
                // table_vec.shrink_to_fit();

                let mut new_vec = Vec::new();
                new_vec.push(new_table);
                if let Some(old_vec) = self.tables.get_mut(&level) {
                    new_vec.append(old_vec);
                }
                self.tables.insert(level, new_vec);
            }
        }
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

    fn key(&self, level: u8, index: u8) -> String {
        return format!("{}_{}", level, index);
    }
}
