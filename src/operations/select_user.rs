use super::Base;
use crate::db_constants::{CURRENT_USER_FILE, DATA_PATH, DIR_PATH, USERS_FILE};
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

pub struct SelectUser<'a> {
    args: &'a [String],
}

impl<'a> SelectUser<'a> {
    pub fn new(args: &'a [String]) -> Self {
        Self { args: args }
    }
}

impl<'a> Base for SelectUser<'a> {
    const OP_NAME: &'static str = "select_user";

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
        let user_list: Vec<&str> = contents.lines().collect();
        if !user_list.contains(&user_name) {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "User does not exist",
            ));
        }

        return Ok(());
    }

    fn perform(&self) -> std::io::Result<()> {
        let user_name: &str = &self.args[1];

        let users_path = format!("{}{}{}", DIR_PATH, DATA_PATH, CURRENT_USER_FILE);
        std::fs::write(&users_path, user_name)?;

        return Ok(());
    }

    fn args(&self) -> &[String] {
        return &(self.args);
    }
}
