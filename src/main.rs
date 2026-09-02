#![allow(warnings)]

use crate::initializer::initialize;
use crate::operations::{Base, ConnectDB, CreateDB, CreateTable, CreateUser, Insert, SelectUser};

use std::any::Any;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::process;
use std::thread;

pub mod db_constants;
pub mod file_ops;
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
        CreateUser::OP_NAME => CreateUser::new(args).op(),
        ConnectDB::OP_NAME => ConnectDB::new(args).op(),
        CreateTable::OP_NAME => CreateTable::new(args).op(),
        SelectUser::OP_NAME => SelectUser::new(args).op(),
        Insert::OP_NAME => Insert::new(args).op(),
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Unrecognized operation",
            ));
        }
    }
}

fn start() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6380")?;

    let pid = process::id();
    println!("Server listening on port 6380. Store some data \u{1F680}");
    for stream in listener.incoming() {
        println!("INCOMING");
        let mut stream = stream?;
        // thread::spawn(move || {
        let mut size_buf = [0u8; 4];
        stream.read_exact(&mut size_buf);
        let size = u32::from_be_bytes(size_buf) as usize;

        println!("SIZE {}", size);
        let mut payload = vec![0u8; size];
        stream.read_exact(&mut payload);
        println!("{:?}", payload);

        /* match operate(buf) {
            Ok(val) => println!("Successful operation!"),
            Err(message) => eprintln!("Got err: {message}"),
        }*/
        // });
    }
    Ok(())
}

fn process_args(args: &[String]) -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6380")?;

    for arg in args {
        println!("WRITING ARG {}", arg);
        let len = arg.len() as u32;
        stream.write_all(&len.to_be_bytes())?;
        stream.write_all(arg.as_bytes())?;
        stream.flush()?;
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && &args[1] == "start" {
        initialize();
        start();
    }

    process_args(&args[1..]);
    /* match operate(&args[1..]) {
        Ok(val) => println!("Successful operation!"),
        Err(message) => eprintln!("Got err: {message}"),
    } */
}
