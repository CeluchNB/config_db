use super::Base;
use crate::db_constants::{DATA_PATH, DIR_PATH, USERS_FILE};
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

pub struct CreateUser<'a> {
    args: &'a [String],
}

impl<'a> CreateUser<'a> {
    pub fn new(args: &'a [String]) -> Self {
        Self { args: args }
    }
}

impl<'a> Base for CreateUser<'a> {
    const OP_NAME: &'static str = "create_user";

    fn validate(&self) -> std::io::Result<()> {
        if self.args.len() == 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Missing user name",
            ));
        }

        if self.args.len() > 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Too many arguments",
            ));
        }

        let user_name: &str = &self.args[1];

        let users_path = format!("{}{}{}", DIR_PATH, DATA_PATH, USERS_FILE);

        let contents = std::fs::read_to_string(&users_path)?;
        let new_contents: Vec<&str> = contents.lines().collect();
        if new_contents.contains(&user_name) {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "User already exists",
            ));
        }

        return Ok(());
    }

    fn perform(&self) -> std::io::Result<()> {
        let user_name: &str = &self.args[1];

        let new_line = format!("{}\n", user_name);

        let users_path = format!("{}{}{}", DIR_PATH, DATA_PATH, USERS_FILE);
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&users_path)?;

        file.write_all(new_line.as_bytes())?;

        return Ok(());
    }

    fn args(&self) -> &[String] {
        return &(self.args);
    }
}
