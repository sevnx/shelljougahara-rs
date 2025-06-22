use shelljougahara::Shell;

#[test]
fn test_history() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let output = shell.execute("history").expect("Failed to execute history");
    assert_eq!(output.0, Some(format!("{:>5} {}", 1, "history")));
}

#[test]
fn test_history_empty() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let output = shell
        .execute("history 0")
        .expect("Failed to execute history");
    assert_eq!(output.0, None);
}

#[test]
fn test_history_limit() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    shell.execute("history").expect("Failed to execute history");
    shell.execute("history").expect("Failed to execute history");
    let output = shell
        .execute("history 2")
        .expect("Failed to execute history");
    assert_eq!(
        output.0,
        Some(format!("{:>5} {}", 1, "history") + "\n" + format!("{:>5} {}", 2, "history").as_str())
    );
}
