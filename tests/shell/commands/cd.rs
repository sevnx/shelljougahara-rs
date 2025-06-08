use shelljougahara::Shell;

#[test]
fn test_cd_root() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let output = shell.execute("cd /").expect("Failed to execute cd");
    assert_eq!(output.0, "".to_string()); // No output expected
    let pwd_output = shell.execute("pwd").expect("Failed to execute pwd");
    assert_eq!(pwd_output.0, "/".to_string());
}

#[test]
fn test_cd_back_and_forth() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    // Go to root
    let output = shell.execute("cd /").expect("Failed to execute cd");
    assert_eq!(output.0, "".to_string()); // No output expected
    let pwd_output = shell.execute("pwd").expect("Failed to execute pwd");
    assert_eq!(pwd_output.0, "/".to_string());
    // Go to home
    let output = shell.execute("cd ~").expect("Failed to execute cd");
    assert_eq!(output.0, "".to_string()); // No output expected
    let pwd_output = shell.execute("pwd").expect("Failed to execute pwd");
    assert_eq!(pwd_output.0, format!("/home/{}", username));
    // Go to parent (home directory)
    let output = shell.execute("cd ..").expect("Failed to execute cd");
    assert_eq!(output.0, "".to_string()); // No output expected
    let pwd_output = shell.execute("pwd").expect("Failed to execute pwd");
    assert_eq!(pwd_output.0, "/home".to_string());
    // Go to current directory (does nothing)
    let output = shell.execute("cd .").expect("Failed to execute cd");
    assert_eq!(output.0, "".to_string()); // No output expected
    let pwd_output = shell.execute("pwd").expect("Failed to execute pwd");
    assert_eq!(pwd_output.0, "/home".to_string());
}

#[test]
fn test_non_existent_directory() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let output = shell
        .execute("cd /non/existent/directory")
        .expect("Failed to execute cd");
    assert_eq!(
        output.0,
        format!("cd: /non/existent/directory: No such file or directory")
    );
}
