use std::error::Error;
use std::fs::{self, File};

use daemonize::Daemonize;

fn main() -> Result<(), Box<dyn Error>> {
    let home = pm3::dir::ensure_pm3_home()?;
    let out_path = home.join("pm3d.out");
    let err_path = home.join("pm3d.err");
    let pid_path = home.join("pm3d.pid");
    let stdout = File::create(&out_path)?;
    let stderr = File::create(&err_path)?;

    Daemonize::new()
        .pid_file(&pid_path)
        .stdout(stdout)
        .stderr(stderr)
        .start()?;

    for i in 1..10 {
        println!("{i}");
        std::thread::sleep(std::time::Duration::from_millis(5000));
    }

    fs::remove_file(out_path)?;
    fs::remove_file(err_path)?;
    fs::remove_file(pid_path)?;

    Ok(())
}
