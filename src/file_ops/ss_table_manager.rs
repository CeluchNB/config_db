use crate::db_constants::{DATA_PATH, DIR_PATH};
use std::fs;
use std::io;

pub struct SSTableManager {}

impl SSTableManager {
    // atomic write commercial
    pub fn write(db_name: &str, table_name: &str, key: &str, element: String) -> io::Result<()> {
        // find correct table
        // insert into table
        // decide whether to merge
        // merge to next level
        Ok(())
    }

    fn current_file() {}

    fn current_tables(db_name: &str, table_name: &str, key: &str) -> io::Result<Vec<String>> {
        let table_dir = format!(
            "{}{}/{}{}/{}",
            DIR_PATH, DATA_PATH, db_name, "/tables", table_name
        );

        let mut files = vec![];
        for entry in fs::read_dir(table_dir)? {
            let entry = entry?;
            let file_type = entry.file_type()?;

            if file_type.is_file() {
                if entry.file_name().to_string_lossy().contains(key) {
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
