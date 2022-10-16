use pm3_lib;

// https://docs.rs/clap/latest/clap/_derive/_cookbook/git/index.html
//
//
use clap::{arg, Command};

fn cli() -> Command {
    Command::new("pm3")
        .about("A process manager for development scripts")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("start")
                .about("Start process by given command")
                .arg(arg!(<COMMAND> "Command to execute"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("stop")
                .about("Stop specified process")
                .arg(arg!(<ID> "Process index"))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("kill").about("Kill daemon and all running processes"))
}

fn main() {
    pm3_lib::dir::ensure_pm3_home().unwrap();
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("start", sub_matches)) => {
            let command = sub_matches.get_one::<String>("COMMAND").unwrap();
            println!("command: {:?}", command);
            start_daemon().unwrap();
        }
        Some(("stop", sub_matches)) => {
            let id = sub_matches.get_one::<String>("ID").unwrap();
            println!("id: {:?}", id);
        }
        Some(("kill", _)) => {
            println!("kill");
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}

fn start_daemon() -> std::io::Result<()> {
    use std::process::{Command, Stdio};
    Command::new("pm3d")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()?;
    Ok(())
}
