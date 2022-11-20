use pm3::rpc::Pm3;
use std::net::SocketAddr;
use tarpc::context;

#[derive(Clone)]
pub struct Pm3Server(SocketAddr);

impl Pm3Server {
    pub fn new(addr: SocketAddr) -> Self {
        Pm3Server(addr)
    }
}

#[tarpc::server]
impl Pm3 for Pm3Server {
    async fn start(self, _: context::Context, command: String) {
        use std::io::{BufRead, BufReader};
        use std::process::{Command, Stdio};

        let child = Command::new(&command)
            .stdout(Stdio::piped())
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();

        let reader = BufReader::new(child.stdout.unwrap());

        reader
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| println!("{}", line));

        format!("executed command");
    }

    async fn get_log(self, _: context::Context) -> Vec<String> {
        return vec![];
    }

    async fn hello(self, _: context::Context, name: String) -> String {
        format!("Hello, {name}! You are connected from {}", self.0)
    }

    async fn ping(self, _: context::Context) {
        ()
    }

    async fn kill(self, _: context::Context) {
        println!("daemon: kill");
        std::process::exit(0);
    }
}
