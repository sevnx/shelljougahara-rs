//! Tests for the echo command.

use shelljougahara::Shell;

#[test]
fn test_echo() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    // Simple test (echo hello)
    let output = shell.execute("echo hello").expect("Failed to execute echo");
    assert_eq!(output.0, Some("hello".to_string()));
    // Test with quotes (echo "hello""hello")
    let output = shell
        .execute("echo \"hello\" \"hello\"")
        .expect("Failed to execute echo");
    assert_eq!(output.0, Some("hello hello".to_string()));
    // Test with quotes (echo "hello" "hello")
    let output = shell
        .execute("echo \"hello\" \"hello\"")
        .expect("Failed to execute echo");
    assert_eq!(output.0, Some("hello hello".to_string()));
}
