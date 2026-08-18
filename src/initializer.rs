use std::fs::{File, OpenOptions, create_dir_all};
use std::io;
use std::path::Path;

const DIR_PATH: &str = "/Users/noah/.configdb";
const DATA_PATH: &str = "/data";
const GLOBAL_WAL_FILE: &str = "/GLOBAL_WAL";
const USERS_FILE: &str = "/USERS";
const REGISTER_FILE: &str = "/REGISTER";

fn data_dir() -> std::io::Result<bool> {
    let base_path = format!("{}{}", DIR_PATH, DATA_PATH);
    let path = Path::new(&base_path);

    if path.is_dir() {
        return Ok(true);
    } else {
        create_dir_all(base_path)?;
    }
    Ok(false)
}

fn user_file() -> std::io::Result<()> {
    let users_path = format!("{}{}{}", DIR_PATH, DATA_PATH, USERS_FILE);
    let mut file = File::create(users_path)?;

    Ok(())
}

fn register_file() -> std::io::Result<()> {
    let register_path = format!("{}{}{}", DIR_PATH, DATA_PATH, REGISTER_FILE);
    let mut file = File::create(register_path);

    Ok(())
}

fn global_wal_file() -> std::io::Result<()> {
    let wal_path = format!("{}{}{}", DIR_PATH, DATA_PATH, GLOBAL_WAL_FILE);
    let mut file = File::create(wal_path);

    Ok(())
}

pub fn initialize() {
    match data_dir() {
        Ok(false) => {
            user_file();
            register_file();
            global_wal_file();
        }
        Ok(true) => {}
        Err(_) => {
            eprintln!("Error initializing service");
        }
    }
}
