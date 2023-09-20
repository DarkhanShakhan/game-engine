use std::{
    error::Error,
    io::{BufRead, BufReader, BufWriter, Write},
    process::{Command, Stdio},
};

pub struct Bot {
    pub name: String,
    child: std::process::Child,
    writer: BufWriter<std::process::ChildStdin>,
    reader: BufReader<std::process::ChildStdout>,
}

impl Bot {
    pub fn new(exec_path: &str, name: &str) -> Bot {
        let mut child = Command::new(exec_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start bot process");

        let writer = BufWriter::new(child.stdin.take().unwrap());
        let reader = BufReader::new(child.stdout.take().unwrap());

        Bot {
            name: name.to_string(),
            child,
            writer,
            reader,
        }
    }

    pub fn send_board(&mut self, board: &str) -> Result<(), Box<dyn Error>> {
        write!(self.writer, "{board}")?;
        self.writer.flush()?;
        Ok(())
    }

    pub fn get_move(&mut self) -> Option<(usize, usize, usize, usize)> {
        let mut move_str = String::new();
        self.reader.read_line(&mut move_str).unwrap();
        if move_str.trim().is_empty() {
            return None;
        }
        let coords: Vec<usize> = move_str
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();
        Some((coords[0], coords[1], coords[2], coords[3]))
    }
}

impl Drop for Bot {
    fn drop(&mut self) {
        self.child.kill().unwrap();
    }
}