// use crate::ds;
use crate::initializer::initialize;
use crate::operations::{Base, ConnectDB, CreateDB, CreateTable, CreateUser, Insert, SelectUser};

use std::env;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process;
use std::thread;

pub mod db_constants;
// pub mod ds;
pub mod file_ops;
pub mod initializer;
pub mod operations;

fn operate(args: &[String]) -> std::io::Result<()> {
    let op = args[0].as_str();
    let new_args: Vec<String> = args[..].to_vec();
    match op {
        CreateDB::OP_NAME => CreateDB::new(&new_args).op(),
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
    println!(
        "Server listening on port 6380. PID: {}. Let's store some data \u{1F680}",
        pid
    );
    for stream in listener.incoming() {
        let mut stream = stream?;

        thread::spawn(move || {
            let mut args: Vec<String> = vec![];
            loop {
                let mut size_buf = [0u8; 4];
                match stream.read_exact(&mut size_buf) {
                    Ok(()) => {}
                    Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                    Err(e) => return Err(e.into()),
                }
                let size = u32::from_be_bytes(size_buf) as usize;

                if size > 1_000_000 {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Requested arg size too big",
                    ));
                }

                let mut payload = vec![0u8; size];
                stream.read_exact(&mut payload)?;

                let str_arg = String::from_utf8(payload).unwrap();
                args.push(str_arg);
            }
            operate(&args)?;
            Ok(())
        });
    }
    Ok(())
}

fn process_args(args: &[String]) -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6380")?;

    for arg in args {
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
        initialize().unwrap();
        start().unwrap();
    }

    process_args(&args[1..]).unwrap();
}
