use std::io::{self,BufRead};
use std::thread;

pub struct UCI {
    #[allow(dead_code)]
    name: String,
}

impl UCI {
    pub fn start() -> u8 {
        let handle = thread::spawn(move || {
            println!("UCI started!");
            let mut line = String::new();
            let stdin = io::stdin();
            stdin.lock().read_line(&mut line).unwrap();
            info!("{}", line);
            line.clear();

            1

        });

        handle.join().unwrap()
    }
}

