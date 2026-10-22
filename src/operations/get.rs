use super::Base;
use crate::in_mem_data::DATA;

pub struct Get<'a> {
    args: &'a [String],
}

impl<'a> Get<'a> {
    pub fn new(args: &'a [String]) -> Self {
        Self { args: args }
    }

    fn search_memory(&self, table_name: &String) -> bool {
        DATA.with(|map_cell| {
            let map = map_cell.borrow_mut();
            let tree = map.get(table_name).unwrap();
            let id_arg = &self.args[2];

            let id: String = String::from(id_arg); // .parse().unwrap();
            match tree.find(id) {
                Some(result) => {
                    println!("RESULT {}", result.borrow().val);
                    return true;
                }
                None => return false,
            }
        });

        return false;
    }
}

impl<'a> Base for Get<'a> {
    const OP_NAME: &'static str = "get";

    fn validate(&self) -> std::io::Result<()> {
        Ok(())
    }

    fn perform(&self) -> std::io::Result<()> {
        let table_name = &self.args[1];
        self.search_memory(table_name);
        Ok(())
    }

    fn args(&self) -> &[String] {
        return &(self.args);
    }
}
