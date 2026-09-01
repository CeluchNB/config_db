use crate::operations::Base;
use std::io;
use std::net::TcpListener;
use std::process;
use std::thread;

pub struct Start<'a> {
    args: &'a [String],
}

impl<'a> Start<'a> {
    pub fn new(args: &'a [String]) -> Self {
        Self { args: args }
    }
}

impl<'a> Base for Start<'a> {
    const OP_NAME: &'static str = "start";
    fn validate(&self) -> std::io::Result<()> {
        Ok(())
    }

    fn perform(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:6380")?;

        let pid = process::id();
        println!("Server listening on port 6380. Store some data \u{1F680}");
        for stream in listener.incoming() {
            let stream = stream?;
            thread::spawn(move || {
                // handle_connection(stream, store);
            });
        }

        Ok(())
    }

    fn args(&self) -> &[String] {
        return &(self.args);
    }
}
