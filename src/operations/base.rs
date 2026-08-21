use crate::db_constants::{DATA_PATH, DIR_PATH, GLOBAL_WAL_FILE};
use std::fs::OpenOptions;
use std::io::Write;

pub trait Base {
    const OP_NAME: &'static str;

    fn op(&self) -> std::io::Result<()> {
        self.validate()?;
        self.write_ahead()?;
        return self.perform();
    }

    fn validate(&self) -> std::io::Result<()>;
    fn perform(&self) -> std::io::Result<()>;
    fn args(&self) -> &[String];

    fn write_ahead(&self) -> std::io::Result<()> {
        let args = self.args();
        let mut command = args.join(" ");
        command.push('\n');

        let wal_path = format!("{}{}{}", DIR_PATH, DATA_PATH, GLOBAL_WAL_FILE);
        let mut file = OpenOptions::new().write(true).append(true).open(wal_path)?;
        file.write_all(command.as_bytes())?;
        Ok(())
    }
}
