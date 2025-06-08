use shelljougahara::Shell;

#[test]
fn test_pwd() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let output = shell.execute("pwd");
    assert_eq!(output.0, format!("/home/{}", username));
}
