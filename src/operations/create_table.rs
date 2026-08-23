use super::Base;
use crate::db_constants::{CURRENT_USER_FILE, DATA_PATH, DIR_PATH};
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

        // let table_name = &self.args[1];

        // Get connected DB dir
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

        let opts: HashMap<String, String> = self.parse_opts()?;

        println!("Table Name: {}", table_name);
        for (opt, val) in &opts {
            println!("Opt: {}, Val: {}", opt, val);
        }
        Ok(())
    }

    fn args(&self) -> &[String] {
        return &(self.args);
    }
}
