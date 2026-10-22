use crate::data_structs::RBTree;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    pub static DATA: RefCell<HashMap<String, RBTree<String, String>>> = RefCell::new(HashMap::new());
}
