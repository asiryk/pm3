use std::error::Error;
use std::fs::File;

use daemonize::Daemonize;

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = File::create("./daemon.out")?;
    let stderr = File::create("./daemon.err")?;

    Daemonize::new()
        .pid_file("./daemon.pid")
        .stdout(stdout)
        .stderr(stderr)
        .start()?;

    for i in 1..10 {
        println!("{i}");
        std::thread::sleep(std::time::Duration::from_millis(5000));
    }

    Ok(())
}
