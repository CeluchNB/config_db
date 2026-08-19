use super::Base;
use std::io;

pub struct CreateDB<'a> {
    args: &'a [String],
}

impl<'a> CreateDB<'a> {
    pub fn new(args: &'a [String]) -> Self {
        Self { args: args }
    }
}

impl<'a> Base for CreateDB<'a> {
    const OP_NAME: &'static str = "create_db";

    fn validate(&self) -> std::io::Result<()> {
        let op: &str = &(self.args[0]);
        return Ok(());
    }

    fn perform(&self) -> std::io::Result<()> {
        let arg: &str = &(self.args[1]);
        return Ok(());
    }

    fn args(&self) -> &[String] {
        return &(self.args);
    }
}
