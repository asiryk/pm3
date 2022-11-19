use log::{info, warn};
use std::io::{BufRead, BufReader};
use std::process::{Child, ChildStdout, Command, Stdio};

pub struct Pool {
    processes: Vec<(Child, BufReader<ChildStdout>)>,
}

impl Pool {
    pub fn new() -> Self {
        Pool { processes: vec![] }
    }

    pub fn spawn(&mut self, command: &str, args: Vec<&str>) {
        info!("start '{}'", command);
        let child = Command::new(command)
            .args(&args)
            .stdin(Stdio::null())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn();

        if let Ok(mut child) = child {
            let stdout = child
                .stdout
                .take()
                .expect("stdout is None on the first take()");
            let reader = BufReader::new(stdout);
            self.processes.push((child, reader));
        } else {
            warn!("could not spawn command: {} - {:?}", command, args);
        }
    }

    pub fn log(&mut self, id: usize) {
        self.processes.get_mut(id).map(|(_, reader)| {
            let lines_iter = reader.lines().filter_map(|line| line.ok());
            let mut counter = 0;
            for line in lines_iter {
                println!("{}", line);
                counter += 1;

                if counter > 2 {
                    break;
                }
            }
        });
    }

    pub fn len(&self) -> usize {
        self.processes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool_add() {
        let mut pool = Pool::new();
        pool.spawn("ping", vec!["127.0.0.1"]);
        assert_eq!(1, pool.len());
    }

    #[test]
    fn pool_log_multiple_times() {
        let mut pool = Pool::new();
        pool.spawn("ping", vec!["127.0.0.1"]);
        pool.log(0);
        pool.log(0);
    }
}
