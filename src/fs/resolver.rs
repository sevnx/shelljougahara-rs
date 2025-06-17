//! File system resolver for paths within it.

use std::path::{Path, PathBuf};

pub fn resolve_path(
    path: &Path,
    home_directory: &Path,
    current_working_directory: &Path,
) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else if path.to_str() == Some("~") {
        home_directory.to_path_buf()
    } else if path.starts_with("~/") {
        home_directory.join(path.strip_prefix("~/").expect("Failed to strip prefix"))
    } else if path.to_str() == Some("..") || path.to_str() == Some("../") {
        if current_working_directory == Path::new("/") {
            Path::new("/").to_path_buf()
        } else {
            current_working_directory.parent().unwrap().to_path_buf()
        }
    } else if path.to_str() == Some(".") || path.to_str() == Some("./") {
        current_working_directory.to_path_buf()
    } else {
        current_working_directory.join(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_absolute_path() {
        let path = Path::new("/usr/bin");
        let home = Path::new("/home/user");
        let cwd = Path::new("/current");

        assert_eq!(resolve_path(path, home, cwd), PathBuf::from("/usr/bin"));
    }

    #[test]
    fn test_home_directory_tilde() {
        let path = Path::new("~");
        let home = Path::new("/home/user");
        let cwd = Path::new("/current");

        assert_eq!(resolve_path(path, home, cwd), PathBuf::from("/home/user"));
    }

    #[test]
    fn test_parent_directory() {
        let path = Path::new("..");
        let home = Path::new("/home/user");
        let cwd = Path::new("/current/dir");

        assert_eq!(resolve_path(path, home, cwd), PathBuf::from("/current"));
    }

    #[test]
    fn test_current_directory() {
        let path = Path::new(".");
        let home = Path::new("/home/user");
        let cwd = Path::new("/current/dir");

        assert_eq!(resolve_path(path, home, cwd), PathBuf::from("/current/dir"));
    }

    #[test]
    fn test_home_relative_path() {
        let path = Path::new("~/documents");
        let home = Path::new("/home/user");
        let cwd = Path::new("/current");

        assert_eq!(
            resolve_path(path, home, cwd),
            PathBuf::from("/home/user/documents")
        );
    }

    #[test]
    fn test_relative_path() {
        let path = Path::new("subdir/file.txt");
        let home = Path::new("/home/user");
        let cwd = Path::new("/current");

        assert_eq!(
            resolve_path(path, home, cwd),
            PathBuf::from("/current/subdir/file.txt")
        );
    }
}
