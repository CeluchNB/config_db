use super::Base;
use crate::db_constants::{CURRENT_USER_FILE, DATA_PATH, DIR_PATH, REGISTER_FILE, TABLE_INFO_FILE};
use crate::file_ops::{Sequence, TableInfo};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

pub struct Insert<'a> {
    args: &'a [String],
}

impl<'a> Insert<'a> {
    pub fn new(args: &'a [String]) -> Self {
        Self { args: args }
    }

    fn current_user(&self) -> std::io::Result<String> {
        let current_user_path = format!("{}{}{}", DIR_PATH, DATA_PATH, CURRENT_USER_FILE);
        let current_user = fs::read_to_string(&current_user_path)?;
        Ok(current_user)
    }

    fn current_db(&self, user: &String) -> std::io::Result<String> {
        let register_path = format!("{}{}{}", DIR_PATH, DATA_PATH, REGISTER_FILE);
        let register_string = fs::read_to_string(&register_path)?;
        for line in register_string.lines() {
            if line.contains(user) {
                let split_line: Vec<&str> = line.split(' ').collect();
                return Ok(String::from(split_line[1]));
            }
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            "User is not connected to DB",
        ))
    }
}

impl<'a> Base for Insert<'a> {
    const OP_NAME: &'static str = "insert";

    fn validate(&self) -> std::io::Result<()> {
        let args = self.args();

        let current_user = self.current_user()?;
        let current_db = self.current_db(&current_user)?;
        let table_name = &args[1];
        // ensure table exists
        let table_dir = format!(
            "{}{}/{}{}/{}",
            DIR_PATH, DATA_PATH, current_db, "/tables", table_name
        );
        let table_path = Path::new(&table_dir);
        if !table_path.is_dir() {
            return Err(io::Error::new(io::ErrorKind::Other, "Table does not exist"));
        }

        let table_info = TableInfo::new(&current_db, table_name);
        let mut expected_fields: Vec<String> = table_info.read_fields()?;
        expected_fields.sort();

        let field_args = &args[2..];
        let mut field_args_vec: Vec<String> = Vec::from(field_args)
            .iter()
            .map(|s| {
                let split_str: Vec<&str> = s.split("=").collect();
                String::from(split_str[0])
            })
            .collect();
        field_args_vec.push(String::from("id"));
        field_args_vec.sort();

        if expected_fields != field_args_vec {
            return Err(io::Error::new(io::ErrorKind::Other, "Fields do not match"));
        }
        Ok(())
    }

    fn perform(&self) -> std::io::Result<()> {
        let args = self.args();

        let current_user = self.current_user()?;
        let current_db = self.current_db(&current_user)?;
        let table_name = &args[1];
        // ensure table exists
        let table_dir = format!(
            "{}{}/{}{}/{}",
            DIR_PATH, DATA_PATH, current_db, "/tables", table_name
        );
        let table_path = Path::new(&table_dir);
        if !table_path.is_dir() {
            return Err(io::Error::new(io::ErrorKind::Other, "Table does not exist"));
        }

        let table_info = TableInfo::new(&current_db, table_name);
        let mut sequence = Sequence::new(&current_db, table_name);
        let id = sequence.increment_sequence()?;

        let field_args = &args[2..];
        let mut field_args_vec: Vec<String> = Vec::from(field_args);
        field_args_vec.sort();
        field_args_vec.insert(0, format!("id={}", id.to_string()));

        let values: Vec<String> = field_args_vec
            .iter()
            .map(|s| {
                let split_str: Vec<&str> = s.split("=").collect();
                String::from(split_str[1])
            })
            .collect();

        println!("{}", values.join(","));
        // generate string row
        //
        // LSM -> insert into memtable (might need more like a storetable)
        //  merge into layer 1
        //  recurse down
        // B-Tree -> find appropriate page
        //  insert
        //  determine if split needed
        Ok(())
    }

    fn args(&self) -> &[String] {
        return &(self.args);
    }
}
