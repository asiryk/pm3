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
