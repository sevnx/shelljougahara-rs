use shelljougahara::Shell;

#[test]
fn test_history() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let output = shell.execute("history").expect("Failed to execute history");
    assert_eq!(output.0, format!("{:>5} {}", 1, "history"));
}
