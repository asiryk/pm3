use std::error::Error;

use daemonize::Daemonize;

fn main() -> Result<(), Box<dyn Error>> {
    Daemonize::new().pid_file("./daemon.pid").start()?;

    for i in 1..10 {
        println!("{i}");
        std::thread::sleep(std::time::Duration::from_millis(5000));
    }

    Ok(())
}
