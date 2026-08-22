use super::Base;
use crate::db_constants::{DATA_PATH, DIR_PATH, REGISTER_FILE};
use std::fs::{OpenOptions, create_dir_all};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub struct ConnectDB<'a> {
    args: &'a [String],
}

impl<'a> ConnectDB<'a> {
    pub fn new(args: &'a [String]) -> Self {
        Self { args: args }
    }
}

impl<'a> Base for ConnectDB<'a> {
    const OP_NAME: &'static str = "connect_db";

    fn validate(&self) -> std::io::Result<()> {
        if self.args.len() < 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Missing user name and/or db name",
            ));
        }

        if self.args.len() > 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Too many arguments. Expected user name and db name.",
            ));
        }

        let user_name: &str = &self.args[1];
        let db_name: &str = &self.args[2];

        let db_file = format!("{}{}/{}", DIR_PATH, DATA_PATH, db_name);
        let db_path = Path::new(&db_file);

        if !db_path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "DB does not exist",
            ));
        }

        return Ok(());
    }

    fn perform(&self) -> std::io::Result<()> {
        let user_name: &str = &self.args[1];
        let db_name: &str = &self.args[2];

        let register_path = format!("{}{}{}", DIR_PATH, DATA_PATH, REGISTER_FILE);

        let contents = std::fs::read_to_string(&register_path)?;
        let mut new_contents: Vec<&str> = contents
            .lines()
            .filter(|line| (*line).contains(db_name))
            .collect();

        let new_line = format!("{} {}", user_name, db_name);
        new_contents.push(&new_line);

        std::fs::write(register_path, new_contents.join("\n"))?;

        return Ok(());
    }

    fn args(&self) -> &[String] {
        return &(self.args);
    }
}
