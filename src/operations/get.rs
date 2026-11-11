use super::Base;
use crate::db_constants::{CURRENT_USER_FILE, DATA_PATH, DIR_PATH, REGISTER_FILE};
use crate::in_mem_data::DATA;
use std::fs;
use std::io;

pub struct Get<'a> {
    args: &'a [String],
}

impl<'a> Get<'a> {
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

    fn search_memory(&self, table_name: &String) -> bool {
        DATA.with(|map_cell| {
            let map = map_cell.borrow_mut();
            let tree = map.get(table_name).unwrap();
            let id_arg = &self.args[2];

            let id: String = String::from(id_arg); // .parse().unwrap();
            match tree.find(id) {
                Some(result) => {
                    println!("RESULT {}", result.borrow().val);
                    return true;
                }
                None => return false,
            }
        });

        return false;
    }

    fn search_ss_table(&self) -> std::io::Result<()> {
        let current_user = self.current_user()?;
        let current_db = self.current_db(&current_user);

        Ok(())
    }
}

impl<'a> Base for Get<'a> {
    const OP_NAME: &'static str = "get";

    fn validate(&self) -> std::io::Result<()> {
        Ok(())
    }

    fn perform(&self) -> std::io::Result<()> {
        let table_name = &self.args[1];
        self.search_memory(table_name);
        Ok(())
    }

    fn args(&self) -> &[String] {
        return &(self.args);
    }
}
