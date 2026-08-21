#![allow(warnings)]

use crate::initializer::initialize;
use crate::operations::{Base, CreateDB};

use std::any::Any;
use std::env;
use std::fs::File;
use std::io;
use std::path::Path;

pub mod db_constants;
pub mod initializer;
pub mod operations;

enum DatabaseImplementation {
    BTree,
    LSMTree,
}

struct DatabaseOptions {
    name: String,
    implementation: DatabaseImplementation,
}

type PrimaryKey = u64;

fn operate(args: &[String]) -> std::io::Result<()> {
    let op: &str = &args[0];

    match op {
        CreateDB::OP_NAME => CreateDB::new(args).op(),
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Unrecognized operation",
            ));
        }
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
