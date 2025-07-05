//! Tests for the touch command.

use shelljougahara::Shell;

#[test]
fn test_touch_file() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let touch = shell
        .execute("touch test")
        .expect("Failed to execute touch");
    assert_eq!(touch.0, None);
}

#[test]
fn test_touch_file_multiple() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let touch = shell
        .execute("touch test1 test2")
        .expect("Failed to execute touch");
    assert_eq!(touch.0, None);
}

#[test]
fn test_touch_and_touch_again() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let touch = shell
        .execute("touch test")
        .expect("Failed to execute touch");
    assert_eq!(touch.0, None);
    let touch2 = shell
        .execute("touch test")
        .expect("Failed to execute touch");
    assert_eq!(touch2.0, None);
}

#[test]
fn test_touch_in_touched_file() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let touch = shell
        .execute("touch test")
        .expect("Failed to execute touch");
    assert_eq!(touch.0, None);
    let touch2 = shell
        .execute("touch test/test2")
        .expect("Failed to execute touch");
    assert_eq!(
        touch2.0,
        Some("touch: cannot touch 'test/test2': Not a directory".to_string())
    );
}

#[test]
fn test_touch_in_non_existent_directory() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let touch = shell
        .execute("touch test/test2")
        .expect("Failed to execute touch");
    assert_eq!(
        touch.0,
        Some("touch: cannot touch 'test/test2': No such file or directory".to_string())
    );
}
