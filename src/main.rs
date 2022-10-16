// https://docs.rs/clap/latest/clap/_derive/_cookbook/git/index.html
//
//
mod cli;

fn main() {
    pm3::dir::ensure_pm3_home().unwrap();
    let matches = cli::init_commands().get_matches();

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
        .stdout(Stdio::inherit())
        .stderr(Stdio::null())
        .spawn()?
        .wait()?;
    Ok(())
}
