use crate::data_structs::{Color, RBNode, RBTree, SSTable};
use crate::initializer::initialize;
use crate::operations::{
    Base, ConnectDB, CreateDB, CreateTable, CreateUser, Get, Insert, SelectUser,
};

use std::cell::RefCell;
use std::env;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process;
use std::rc::Rc;
// use std::thread;

pub mod data_structs;
pub mod db_constants;
pub mod file_ops;
pub mod in_mem_data;
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
        Get::OP_NAME => Get::new(args).op(),
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

        // thread::spawn(move || {
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
        // Ok(())
        // });
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
    /* let first_root = Rc::new(RefCell::new(RBNode::new(
        String::from("apple"),
        String::from("testapple"),
        Color::Black,
        None,
    )));
    let mut first_tree = RBTree::new(first_root);

    let nodes = vec![
        String::from("cat"),
        String::from("elephant"),
        String::from("ghost"),
        String::from("igloo"),
        String::from("kite"),
        String::from("mom"),
        String::from("ocelot"),
        String::from("quiet"),
        String::from("stock"),
    ];
    for node in &nodes {
        first_tree
            .insert(String::from(node), format!("test{}", String::from(node)))
            .unwrap();
    }

    let first_table = SSTable::new("users_1", "books", "id", 1, 1, &first_tree).unwrap();

    let second_root = Rc::new(RefCell::new(RBNode::new(
        String::from("banana"),
        String::from("testbanana"),
        Color::Black,
        None,
    )));
    let mut second_tree = RBTree::new(second_root);
    let second_nodes = vec![
        String::from("dog"),
        String::from("fog"),
        String::from("house"),
        String::from("joker"),
        String::from("louse"),
        String::from("nope"),
        String::from("power"),
        String::from("roast"),
        String::from("thunder"),
    ];
    for node in &second_nodes {
        second_tree
            .insert(String::from(node), format!("test{}", String::from(node)))
            .unwrap();
    }
    let second_table = SSTable::new("users_1", "books", "id", 1, 2, &second_tree).unwrap();

    let merged = first_table.merge(&second_table).unwrap();
    for offset in merged.offsets {
        println!("Offset {}", offset);
    }*/

    /* for offset in new_table.offsets {
        println!("{}", offset);
    }*/

    /* let initial_root = tree.find(10);
    initial_root.unwrap().borrow().print_me();
    for node in &nodes {
        let n = tree.find(*node);
        n.unwrap().borrow().print_me();
    }*/

    let string = "test_l113_9";
    let elements: Vec<&str> = string.split("_").collect();
    let result: [u8; 2] = [
        elements[1][1..].parse().unwrap(),
        elements[2].parse().unwrap(),
    ];
    println!("One {}, Two {}", result[0], result[1]);

    /* let args: Vec<String> = env::args().collect();
    if args.len() == 2 && &args[1] == "start" {
        initialize().unwrap();
        start().unwrap();
    }

    process_args(&args[1..]).unwrap();*/
}
