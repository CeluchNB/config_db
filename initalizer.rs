use std::fs::File;
use std::path::Path;

const DIR_PATH: &str = "~/.configdb";
const DATA_PATH: &str = "/data";

fn base() {}

fn users() {}

pub fn initialize() {
    base();
    users();
}
