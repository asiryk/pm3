mod cli;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};

use log::{error, info, warn};
use tarpc::context;

use crate::rpc_client::{api_test, bootsrap, create_client};

const PORT: u16 = 9876;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    pm3::dir::ensure_pm3_home().unwrap();
    handle_matches().await;
}

mod rpc_client {
    use log::info;
    use pm3::rpc::Pm3Client;
    use std::error::Error;
    use std::net::SocketAddr;
    use tarpc::{client, context, serde_transport::tcp, tokio_serde::formats::Json};

    use crate::start_daemon;

    pub async fn create_client(addr: SocketAddr) -> std::io::Result<Pm3Client> {
        let transport = tcp::connect(addr, Json::default).await?;
        let client = Pm3Client::new(client::Config::default(), transport).spawn();
        Ok(client)
    }

    pub async fn bootsrap(addr: SocketAddr) -> Result<Pm3Client, Box<dyn Error>> {
        let transport = tcp::connect(addr, Json::default).await;
        match transport {
            Ok(transport) => {
                let client = Pm3Client::new(client::Config::default(), transport).spawn();
                info!("connected to daemon");
                return Ok(client);
            }
            Err(_) => {
                info!("daemon is not running, starting it");
                start_daemon().await?;
                let transport = tcp::connect(addr, Json::default).await?;
                let client = Pm3Client::new(client::Config::default(), transport).spawn();
                info!("connected to daemon");
                return Ok(client);
            }
        }
    }

    pub async fn api_test(client: &Pm3Client) {
        let msg = client
            .hello(context::current(), format!("{}", "smth"))
            .await
            .unwrap();
        println!("{}", msg);
    }
}

async fn start_daemon() -> std::io::Result<()> {
    use std::process::Stdio;
    use tokio::process::Command;
    Command::new("pm3d")
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::null())
        .spawn()?
        .wait()
        .await?;
    // TODO: need to await some time until tokio runtime is spawned on daemon side
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    Ok(())
}

async fn handle_matches() {
    let matches = cli::init_commands().get_matches();
    let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), PORT);

    match matches.subcommand() {
        Some(("start", sub_matches)) => {
            let command = sub_matches.get_one::<String>("COMMAND").unwrap();
            println!("command: {:?}", command);
        }
        Some(("stop", sub_matches)) => {
            let id = sub_matches.get_one::<String>("ID").unwrap();
            println!("id: {:?}", id);
        }
        Some(("boot", _)) => {
            let client = bootsrap(addr).await;
            info!("client bootstrapped");
            let client = client.unwrap();
            api_test(&client).await;
        }
        Some(("kill", _)) => {
            let client = create_client(addr).await;
            match client {
                Ok(client) => {
                    let _ = client.kill(context::current()).await;
                    info!("successfully killed daemon");
                }
                Err(error) => {
                    if let std::io::ErrorKind::ConnectionRefused = error.kind() {
                        info!("daemon was not active, did nothing");
                    } else {
                        warn!("unexpected error when killing daemon");
                        error!("{:?}", error);
                    }
                }
            }
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable !()
    }
}
