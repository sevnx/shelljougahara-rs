//! Tests for the ls command.

use shelljougahara::Shell;

#[test]
fn test_ls_empty_directory() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let ls = shell.execute("ls").expect("Failed to execute ls");
    assert_eq!(ls.0, None);
}

#[test]
fn test_ls_created_directory() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let mkdir = shell
        .execute("mkdir test")
        .expect("Failed to execute mkdir");
    assert_eq!(mkdir.0, None);
    let ls = shell.execute("ls test").expect("Failed to execute ls");
    assert_eq!(ls.0, None);
}

#[test]
fn test_ls_unexisting_directory() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let ls = shell.execute("ls test").expect("Failed to execute ls");
    assert_eq!(
        ls.0,
        Some("ls: cannot access 'test': No such file or directory".to_string())
    );
}

#[test]
fn test_ls_file() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let mkdir = shell
        .execute("touch test")
        .expect("Failed to execute touch");
    assert_eq!(mkdir.0, None);
    let ls = shell.execute("ls test").expect("Failed to execute ls");
    assert_eq!(ls.0, Some("test".to_string()));
}

#[test]
fn test_ls_single_directory() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let mkdir = shell
        .execute("mkdir test")
        .expect("Failed to execute mkdir");
    assert_eq!(mkdir.0, None);
    let ls = shell.execute("ls").expect("Failed to execute ls");
    assert_eq!(ls.0, Some("test".to_string()));
}
