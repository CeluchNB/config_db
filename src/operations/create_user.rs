use super::Base;
use crate::db_constants::{DATA_PATH, DIR_PATH, USERS_FILE};
use std::fs::OpenOptions;
use std::io::{self, Write};

pub struct CreateUser {
    args: Vec<String>,
}

impl CreateUser {
    pub fn new(args: Vec<String>) -> Self {
        Self { args: args }
    }
}

impl Base for CreateUser {
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

        let user_name: String = self.args[1].clone();

        let users_path = format!("{}{}{}", DIR_PATH, DATA_PATH, USERS_FILE);

        let contents = std::fs::read_to_string(&users_path)?;
        let new_contents: Vec<String> = contents.lines().map(|s| String::from(s)).collect();
        if new_contents.contains(&user_name) {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "User already exists",
            ));
        }

        return Ok(());
    }

    fn perform(&self) -> std::io::Result<()> {
        let user_name: String = self.args[1].clone();

        let new_line = format!("{}\n", user_name);

        let users_path = format!("{}{}{}", DIR_PATH, DATA_PATH, USERS_FILE);
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&users_path)?;

        file.write_all(new_line.as_bytes())?;

        return Ok(());
    }

    fn args(&self) -> &Vec<String> {
        return &(self.args);
    }
}
