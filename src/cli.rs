use clap::{arg, Command};

pub fn init_commands() -> Command {
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
