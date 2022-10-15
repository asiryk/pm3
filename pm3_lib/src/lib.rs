pub mod dir {
    use std::path::PathBuf;

    pub fn home_dir() -> Option<PathBuf> {
        return std::env::var_os("HOME")
            .and_then(|h| if h.is_empty() { None } else { Some(h) })
            .map(PathBuf::from);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let path = dir::home_dir();
        assert!(path.is_some())
    }
}
