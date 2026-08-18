use std::fs::File;
use std::path::Path;

const DIR_PATH: &str = "~/.configdb";
const DATA_PATH: &str = "/data";

fn base() {
    let path = format!("{DIR_PATH}{DATA_PATH}");
}

fn users() {}

pub fn initialize() {
    base();
    users();
}
