use log::{error, info, warn};
use std::io::{BufRead, BufReader};
use std::process::{Child, ChildStdout, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::log_buffer::{ClientId, LogBuffer};

// TODO: rewrite with tokio mutex and async
struct Process {
    child: Child,
    log_buffer: Arc<Mutex<LogBuffer>>,
}

impl Process {
    pub fn new(child: Child, reader: BufReader<ChildStdout>) -> Self {
        let log_buffer = Arc::new(Mutex::new(LogBuffer::new()));
        let t_log_buffer = Arc::clone(&log_buffer);

        thread::spawn(move || {
            let lines = reader.lines().filter_map(|line| line.ok());

            // TODO: seems like this thred will live untill process kill
            for line in lines {
                let mut log_buffer = match t_log_buffer.lock() {
                    Ok(i) => i,
                    Err(i) => i.into_inner(),
                };

                log_buffer.write(line);
            }
        });

        return Process { child, log_buffer };
    }

    pub fn log(&self, id: &ClientId) -> Vec<String> {
        let mut log_buffer = match self.log_buffer.lock() {
            Ok(i) => i,
            Err(i) => i.into_inner(),
        };
        return log_buffer.consume_unread(id);
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        // safe to unwrap because child is not killed in any other place.
        self.child.kill().unwrap();
    }
}

pub struct Pool {
    processes: Vec<Option<Process>>,
}

impl Pool {
    pub fn new() -> Self {
        Pool { processes: vec![] }
    }

    pub fn spawn(&mut self, command: &str, args: &[&str]) {
        info!("start '{}'", command);
        let child = Command::new(command)
            .args(args)
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .stdout(Stdio::piped())
            .spawn();

        if let Ok(mut child) = child {
            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);
                self.processes.push(Some(Process::new(child, reader)));
            } else {
                // it should not happen since we take() for the first time.
                error!("could not spawn command: no stdout handle");
            }
        } else {
            error!("could not spawn command: {} - {:?}", command, args);
        }
    }

    pub fn log(&mut self, pid: usize, cid: ClientId) -> Option<Vec<String>> {
        let process = self.processes.get_mut(pid)?;

        return if let Some(process) = process {
            process.log(&cid).into()
        } else {
            None
        };
    }

    pub fn stop(&mut self, pid: usize) {
        if let Some(process) = self.processes.get_mut(pid) {
            if let Some(process) = process.take() {
                drop(process);
            } else {
                warn!("process '{}' is already stopped", pid);
            }
        }
    }

    pub fn len(&self) -> usize {
        return self.processes.iter().filter(|p| p.is_some()).count();
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    #[ignore]
    fn pool_add() {
        let mut pool = Pool::new();
        pool.spawn("ping", &["127.0.0.1"]);
        let l = pool.log(0, ClientId(0)).unwrap();
        pool.log(0, ClientId(1)).unwrap();
        println!("{:?}", l);

        thread::sleep(Duration::from_millis(5000));

        let l = pool.log(0, ClientId(0)).unwrap();
        println!("{:?}", l);

        thread::sleep(Duration::from_millis(2000));

        let l = pool.log(0, ClientId(0)).unwrap();
        println!("{:?}", l);

        let l = pool.log(0, ClientId(0)).unwrap();
        println!("{:?}", l);

        let l = pool.log(0, ClientId(0)).unwrap();
        println!("{:?}", l);

        let l = pool.log(0, ClientId(1)).unwrap();
        println!("{:?}", l);
    }
}
