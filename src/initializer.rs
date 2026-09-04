use super::db_constants::{DATA_PATH, DIR_PATH, GLOBAL_WAL_FILE, REGISTER_FILE, USERS_FILE};
use std::fs::{File, create_dir_all};
use std::io;
use std::path::Path;

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
    File::create(users_path)?;

    Ok(())
}

fn register_file() -> std::io::Result<()> {
    let register_path = format!("{}{}{}", DIR_PATH, DATA_PATH, REGISTER_FILE);
    File::create(register_path)?;

    Ok(())
}

fn global_wal_file() -> std::io::Result<()> {
    let wal_path = format!("{}{}{}", DIR_PATH, DATA_PATH, GLOBAL_WAL_FILE);
    File::create(wal_path)?;

    Ok(())
}

pub fn initialize() -> std::io::Result<()> {
    match data_dir() {
        Ok(false) => {
            user_file()?;
            register_file()?;
            global_wal_file()?;
            Ok(())
        }
        Ok(true) => Ok(()),
        Err(_) => {
            eprintln!("Error initializing service");
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Could not initialize service",
            ))
        }
    }
}
