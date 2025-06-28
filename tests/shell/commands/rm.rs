//! Tests for the rm command.

use shelljougahara::Shell;

#[test]
fn test_simple_rm() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let mkdir = shell
        .execute("mkdir test")
        .expect("Failed to execute mkdir");
    assert_eq!(mkdir.0, None);
    let rm = shell.execute("rm -rf test").expect("Failed to execute rm");
    assert_eq!(rm.0, None);
    // TODO: Replace by ls once implemented
    let cd = shell.execute("cd test").expect("Failed to execute cd");
    assert!(cd.0.is_some());
}

#[test]
fn test_multiple_rm() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let mkdir = shell
        .execute("mkdir test")
        .expect("Failed to execute mkdir");
    assert_eq!(mkdir.0, None);
    let rm = shell.execute("rm -rf test").expect("Failed to execute rm");
    assert_eq!(rm.0, None);
    let mkdir = shell
        .execute("mkdir test")
        .expect("Failed to execute mkdir");
    assert_eq!(mkdir.0, None);
    let rm = shell.execute("rm -rf test").expect("Failed to execute rm");
    assert_eq!(rm.0, None);
}
