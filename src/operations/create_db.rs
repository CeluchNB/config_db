use super::Base;
use crate::db_constants::{DATA_PATH, DIR_PATH};
use std::fs::create_dir_all;
use std::io;
use std::path::Path;

pub struct CreateDB<'a> {
    args: &'a [String],
}

impl<'a> CreateDB<'a> {
    pub fn new(args: &'a [String]) -> Self {
        Self { args: args }
    }
}

impl<'a> Base for CreateDB<'a> {
    const OP_NAME: &'static str = "create_db";

    fn validate(&self) -> std::io::Result<()> {
        if self.args.len() == 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Missing database name",
            ));
        }

        if self.args.len() > 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Too many arguments",
            ));
        }

        let table_name = format!("/{}", &(self.args[1]));

        let table_dir = format!("{}{}{}", DATA_PATH, DIR_PATH, table_name);
        let table_path = Path::new(&table_dir);

        if table_path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Database already exists",
            ));
        }

        return Ok(());
    }

    fn perform(&self) -> std::io::Result<()> {
        let table_name = format!("/{}", &(self.args[1]));

        let table_dir = format!("{}{}{}/tables", DIR_PATH, DATA_PATH, table_name);
        let table_path = Path::new(&table_dir);

        create_dir_all(table_path);

        return Ok(());
    }

    fn args(&self) -> &[String] {
        return &(self.args);
    }
}
