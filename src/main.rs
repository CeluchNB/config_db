use std::any::Any;
use std::fs::File;

enum DatabaseImplementation {
    BTree,
    LSMTree,
}

struct DatabaseOptions {
    name: String,
    implementation: DatabaseImplementation,
}

trait ConfigDBOps {
    fn create_database(&self, name: String) -> Result<bool, &str>;
    fn delete_database(&self, name: String) -> Result<bool, &str>;
}

struct ConfigDB {}

impl ConfigDBOps for ConfigDB {
    fn create_database(&self, name: String) -> Result<bool, &str> {
        return Ok(true);
    }

    fn delete_database(&self, name: String) -> Result<bool, &str> {
        return Ok(true);
    }
}

struct DatabaseProperties {
    root_path: String,
    wal: WAL,
}

pub trait DatabaseOps {
    fn add_table(&self, name: String, fields: Vec<(String, String)>) -> Result<bool, &str>;
    fn delete_table(&self, name: String) -> Result<bool, &str>;
}

struct Database<T: DatabaseTable> {
    tables: Vec<T>,
}

impl<T: DatabaseTable> DatabaseOps for Database<T> {
    fn add_table(&self, name: String, fields: Vec<(String, String)>) -> Result<bool, &str> {
        return Ok(true);
    }

    fn delete_table(&self, name: String) -> Result<bool, &str> {
        return Ok(true);
    }
}

type PrimaryKey = u64;

pub trait DatabaseTable {
    fn insert(&self, row: &dyn Any);
    fn update(&self, key: PrimaryKey, row: &dyn Any);
    fn get(&self, key: PrimaryKey) -> Option<Box<dyn Any>>;
    fn delete(&self, key: PrimaryKey) -> Option<Box<dyn Any>>;
}

struct LSMTreeDatabaseTable {}

impl DatabaseTable for LSMTreeDatabaseTable {
    fn insert(&self, row: &dyn Any) {}

    fn update(&self, key: PrimaryKey, row: &dyn Any) {}

    fn get(&self, key: PrimaryKey) -> Option<Box<dyn Any>> {
        return None;
    }

    fn delete(&self, key: PrimaryKey) -> Option<Box<dyn Any>> {
        return None;
    }
}

struct WAL {
    file_path: String,
    file: File,
}

fn main() {
    println!("Hello, world!");
}
