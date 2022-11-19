use std::error::Error;

use daemonize::Daemonize;

fn main() -> Result<(), Box<dyn Error>> {
    Daemonize::new().start()?;

    // Build tokio runtime after daemonizing. It causes a problem that client tries to
    // connect to the server while the runtime or the server didn't not spawn yet.
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            rpc_server::start_rpc_server().await.unwrap();
        });

    Ok(())
}

mod rpc_server {
    use futures::{future, prelude::*};
    use pm3::rpc::Pm3;
    use std::error::Error;
    use std::net::{IpAddr, Ipv6Addr, SocketAddr};
    use tarpc::{
        context,
        server::{self, incoming::Incoming, Channel},
        tokio_serde::formats::Json,
    };

    #[derive(Clone)]
    pub struct Pm3Server(SocketAddr);

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

    pub async fn start_rpc_server() -> Result<(), Box<dyn Error>> {
        let server_addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 9876);
        let transport = tarpc::serde_transport::tcp::listen(server_addr, Json::default);
        let mut listener = transport.await?;
        listener.config_mut().max_frame_length(usize::MAX);
        listener
            .filter_map(|r| future::ready(r.ok()))
            .map(server::BaseChannel::with_defaults)
            .max_channels_per_key(1, |t| t.transport().peer_addr().unwrap().ip())
            .map(|channel| {
                let server = Pm3Server(channel.transport().peer_addr().unwrap());
                channel.execute(server.serve())
            })
            .buffer_unordered(10)
            .for_each(|_| async {})
            .await;

        Ok(())
    }
}

mod cmd {
    use log::warn;
    use std::cell::Cell;
    use std::fs::File;
    use std::io;
    use std::process::{Child, Command, Stdio};
    use std::time;

    pub struct Pm3Command {
        command: String,
        process: Cell<Option<Child>>,
    }

    impl Pm3Command {
        pub fn init(command: String) -> Self {
            Pm3Command {
                command,
                process: Cell::new(None),
            }
        }

        pub fn spawn(&self) -> io::Result<&Self> {
            let mut log_path = pm3::dir::pm3_log_dir()?;
            // TODO: USE CHRONO crate
            log_path.push("cmd_name_with_timestamp.log");
            let log_file = File::create(log_path)?;

            let child = Command::new(&self.command)
                .stdout(Stdio::piped())
                .stdin(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?;
            self.process.set(Some(child));
            Ok(self)
        }

        pub fn kill(&self) -> io::Result<&Self> {
            if let Some(mut process) = self.process.take() {
                process.kill()?;
            } else {
                warn!(
                    "Could not kill child process since it's not running '{}'",
                    &self.command
                );
            }
            Ok(self)
        }
    }
}
