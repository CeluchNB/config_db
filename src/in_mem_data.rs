use crate::data_structs::{RBTree, SSTable};
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    pub static DATA: RefCell<HashMap<String, RBTree<String, String>>> = RefCell::new(HashMap::new());
    pub static DISK_DATA: RefCell<HashMap<String, SSTable<String, String>>> = RefCell::new(HashMap::new());
}
