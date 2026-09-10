use crate::db_constants::{DATA_PATH, DIR_PATH, TABLE_INFO_FILE};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

pub struct TableInfo {
    table_info_file: File,
}

impl TableInfo {
    pub fn new(db_name: &str, table_name: &str) -> Self {
        let dir = format!(
            "{}{}/{}/{}/{}",
            DIR_PATH, DATA_PATH, db_name, "tables", table_name
        );
        fs::create_dir_all(&dir).unwrap();

        let file_path = format!("{}{}", dir, TABLE_INFO_FILE);
        let table_info_result = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(&file_path);

        match table_info_result {
            Ok(file) => Self {
                table_info_file: file,
            },
            Err(e) => panic!("Table info file not created: {}", e),
        }
    }

    pub fn initialize_file(
        &mut self,
        implementation: &str,
        index: &str,
        fields: &[String],
    ) -> std::io::Result<()> {
        let impl_string = format!("Implementation={}\n", implementation);
        let index_string = format!("Index={}\n", index);
        let field_header: Vec<String> = vec![String::from("Fields"), String::from("id")];
        let full_fields: String = [field_header.as_slice(), fields].concat().join("\n");

        self.table_info_file.write_all(impl_string.as_bytes())?;
        self.table_info_file.write_all(index_string.as_bytes())?;
        self.table_info_file.write_all(full_fields.as_bytes())?;
        Ok(())
    }

    pub fn read_implementation(&self) -> std::io::Result<String> {
        let reader = BufReader::new(&self.table_info_file);
        for line_result in reader.lines() {
            let line: String = line_result?;
            if line.starts_with("Implementation") {
                let result: Vec<&str> = line.split("=").collect();
                return Ok(String::from(result[1]));
            }
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            "No implementation found",
        ))
    }

    pub fn read_index(&self) -> std::io::Result<String> {
        let reader = BufReader::new(&self.table_info_file);
        for line_result in reader.lines() {
            let line: String = line_result?;
            if line.starts_with("Index") {
                let result: Vec<&str> = line.split("=").collect();
                return Ok(String::from(result[1]));
            }
        }

        Err(io::Error::new(io::ErrorKind::Other, "No index found"))
    }

    pub fn read_fields(&self) -> std::io::Result<Vec<String>> {
        let reader = BufReader::new(&self.table_info_file);
        let mut collect_fields = false;
        let mut result: Vec<String> = vec![];

        for line_result in reader.lines() {
            let line = line_result?;
            if line == "Fields" {
                collect_fields = true;
                continue;
            }
            if collect_fields {
                result.push(line);
            }
        }
        Ok(result)
    }
}
