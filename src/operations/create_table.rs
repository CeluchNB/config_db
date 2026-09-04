use super::Base;
use crate::db_constants::{CURRENT_USER_FILE, DATA_PATH, DIR_PATH, REGISTER_FILE};
use crate::file_ops::{Sequence, TableInfo};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

pub struct CreateTable<'a> {
    args: &'a [String],
}

impl<'a> CreateTable<'a> {
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

    fn parse_opts(&self) -> std::io::Result<HashMap<String, String>> {
        let mut opts = HashMap::new();

        let opt_input = &self.args[2..];
        let mut i = 0;
        // --impl b --idx name --fields name email score
        while i < opt_input.len() {
            if opt_input[i] == "--impl" {
                opts.insert(String::from("--impl"), String::from(&opt_input[i + 1]));
                i += 2;
                continue;
            }
            if opt_input[i] == "--fields" {
                let field_names: Vec<String> = opt_input
                    .iter()
                    .skip(i + 1)
                    .take_while(|arg| !arg.starts_with("--"))
                    .cloned()
                    .collect();

                let field_string = field_names.join(" ");
                opts.insert(String::from("--fields"), String::from(field_string));
                i += field_names.len() + 1;
                continue;
            }
            if opt_input[i] == "--idx" {
                opts.insert(String::from("--idx"), String::from(&opt_input[i + 1]));
                i += 2;
                continue;
            }
        }

        Ok(opts)
    }
}

impl<'a> Base for CreateTable<'a> {
    const OP_NAME: &'static str = "create_table";

    fn validate(&self) -> std::io::Result<()> {
        if self.args.len() == 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Missing table name",
            ));
        }

        let current_user = self.current_user()?;
        if current_user.len() == 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "No active user"));
        }

        let current_db = self.current_db(&current_user)?;

        let table_dir = format!(
            "{}{}/{}/{}/{}",
            DIR_PATH,
            DATA_PATH,
            current_db,
            "tables",
            &(self.args[1])
        );
        let table_path = Path::new(&table_dir);

        if table_path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Table already exists",
            ));
        }

        let opts = self.parse_opts()?;
        let field_opt: &str = opts.get("--fields").map(|o| o.as_str()).unwrap_or("");
        if field_opt.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Must have at least one field",
            ));
        }

        return Ok(());
    }

    fn perform(&self) -> std::io::Result<()> {
        let current_user = self.current_user()?;
        let current_db = self.current_db(&current_user)?;
        let opts = self.parse_opts()?;

        let impl_opt: &str = opts.get("--impl").map(|o| o.as_str()).unwrap_or("b");
        let idx_opt: &str = opts.get("--idx").map(|o| o.as_str()).unwrap_or("");
        let field_opt: &String = opts.get("--fields").unwrap();
        let fields: Vec<&str> = field_opt.split(" ").collect();

        let mut table_info = TableInfo::new(&current_db, &(self.args[1]));
        table_info.initialize_file(impl_opt, idx_opt, fields)?;

        let mut sequence_file = Sequence::new(&current_db, &(self.args[1]));
        sequence_file.initialize_file()?;

        Ok(())
    }

    fn args(&self) -> &[String] {
        return &(self.args);
    }
}
