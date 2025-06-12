use shelljougahara::{Shell, ShellError};

#[test]
fn test_exit() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    shell.execute("exit").expect("Failed to execute exit");
    let try_output = shell.execute("echo hello");
    assert_eq!(try_output.unwrap_err(), ShellError::ShellNotActive);
}
