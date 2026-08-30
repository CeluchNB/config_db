use crate::db_constants::{DATA_PATH, DIR_PATH, SEQUENCE_FILE};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};

pub struct Sequence {
    sequence_file: File,
}

impl Sequence {
    pub fn new(db_name: &str, table_name: &str) -> Self {
        let dir = format!(
            "{}{}/{}/{}/{}",
            DIR_PATH, DATA_PATH, db_name, "tables", table_name
        );
        fs::create_dir_all(&dir);

        let file_path = format!("{}{}", dir, SEQUENCE_FILE);
        let table_info_result = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&file_path);

        match table_info_result {
            Ok(file) => Self {
                sequence_file: file,
            },
            Err(e) => panic!("Sequence file not created: {}", e),
        }
    }

    pub fn initialize_file(&mut self) -> std::io::Result<()> {
        &self.sequence_file.write_all(b"0");

        Ok(())
    }

    pub fn increment_sequence(&mut self) -> std::io::Result<u32> {
        &self.sequence_file.lock();

        let mut current_value = String::new();
        &self.sequence_file.read_to_string(&mut current_value);
        let mut id: u32 = current_value.parse().unwrap();
        id += 1;

        &self.sequence_file.seek(SeekFrom::Start(0))?;
        &self.sequence_file.set_len(0)?;
        &self.sequence_file.write_all(format!("{}", id).as_bytes());

        &self.sequence_file.unlock();
        Ok(id)
    }
}
