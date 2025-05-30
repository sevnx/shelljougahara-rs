use shelljougahara::Shell;

#[test]
fn test_pwd() {
    let mut shell = Shell::default();
    let output = shell.execute("pwd");
    assert_eq!(output.0, "/");
}
