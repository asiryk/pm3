pub mod dir {
    use std::io::{Error, ErrorKind, Result};
    use std::path::PathBuf;

    pub fn home_dir() -> Result<PathBuf> {
        let err = Error::new(
            ErrorKind::NotFound,
            "platform-specific $HOME directory not found",
        );
        return std::env::var_os("HOME")
            .and_then(|h| if h.is_empty() { None } else { Some(h) })
            .ok_or(err)
            .map(PathBuf::from);
    }

    pub fn ensure_pm3_home() -> Result<PathBuf> {
        let path = home_dir()?.join(".pm3");
        if !path.exists() {
            std::fs::create_dir(&path)?;
        }

        Ok(path)
    }

    pub fn pm3_log_dir() -> Result<PathBuf> {
        let mut path = ensure_pm3_home()?;
        path.push("log");

        Ok(path)
    }
}

pub mod rpc {
    #[tarpc::service]
    pub trait Pm3 {
        async fn start(command: String, args: Vec<String>);
        async fn get_log() -> Vec<String>;
        async fn hello(name: String) -> String;
        async fn ping();
        async fn kill();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_dir() {
        let path = dir::home_dir();
        assert!(path.is_ok())
    }

    #[test]
    fn ensure_pm3_home() {
        let path = dir::ensure_pm3_home();
        assert!(path.is_ok())
    }
}
