#![allow(warnings)]

use crate::initializer::initialize;
// use crate::operations::CreateDB;

use std::any::Any;
use std::env;
use std::fs::File;
use std::path::Path;

pub mod initializer;

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

trait DBOperation {
    const OP_NAME: &'static str;

    fn op(&self) -> Result<(), String> {
        match self.validate() {
            Ok(()) => (),
            Err(message) => return Err(message),
        }
        self.write_ahead();
        return self.perform();
    }

    fn validate(&self) -> Result<(), String>;
    fn perform(&self) -> Result<(), String>;
    fn args(&self) -> &[String];

    fn write_ahead(&self) {
        let args = self.args();
        let op: &str = &args[0];
        // Write to WAL
        println!("Validating: {op}");
    }
}

struct CreateDBOperation<'a> {
    args: &'a [String],
}

impl<'a> DBOperation for CreateDBOperation<'a> {
    const OP_NAME: &'static str = "create_db";

    fn validate(&self) -> Result<(), String> {
        let op: &str = &(self.args[0]);
        println!("Validating: {op}");
        return Ok(());
    }

    fn perform(&self) -> Result<(), String> {
        let arg: &str = &(self.args[1]);
        println!("Creating DB: {arg}");
        return Ok(());
    }

    fn args(&self) -> &[String] {
        return &(self.args);
    }
}

fn operate(args: &[String]) -> Result<(), String> {
    let op: &str = &args[0];
    let name = CreateDBOperation::OP_NAME;

    println!("OP NAME {name} {op}");
    match op {
        CreateDBOperation::OP_NAME => CreateDBOperation { args: &args }.op(),
        _ => Err("Invalid operation".to_string()),
    }
}

fn main() {
    initialize();

    let args: Vec<String> = env::args().collect();
    match operate(&args[1..]) {
        Ok(val) => println!("Successful operation!"),
        Err(message) => eprintln!("Got err: {message}"),
    }
}
