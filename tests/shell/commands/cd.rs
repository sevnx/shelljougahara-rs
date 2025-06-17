use shelljougahara::Shell;

#[test]
fn test_cd_root() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let output = shell.execute("cd /").expect("Failed to execute cd");
    assert_eq!(output.0, None);
    let pwd_output = shell.execute("pwd").expect("Failed to execute pwd");
    assert_eq!(pwd_output.0, Some("/".to_string()));
}

#[test]
fn test_cd_back_and_forth() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    // Go to root
    let output = shell.execute("cd /").expect("Failed to execute cd");
    assert_eq!(output.0, None);
    let pwd_output = shell.execute("pwd").expect("Failed to execute pwd");
    assert_eq!(pwd_output.0, Some("/".to_string()));
    // Go to home
    let output = shell.execute("cd ~").expect("Failed to execute cd");
    assert_eq!(output.0, None);
    let pwd_output = shell.execute("pwd").expect("Failed to execute pwd");
    assert_eq!(pwd_output.0, Some(format!("/home/{username}")));
    // Go to parent (home directory)
    let output = shell.execute("cd ..").expect("Failed to execute cd");
    assert_eq!(output.0, None);
    let pwd_output = shell.execute("pwd").expect("Failed to execute pwd");
    assert_eq!(pwd_output.0, Some("/home".to_string()));
    // Go to current directory (does nothing)
    let output = shell.execute("cd .").expect("Failed to execute cd");
    assert_eq!(output.0, None);
    let pwd_output = shell.execute("pwd").expect("Failed to execute pwd");
    assert_eq!(pwd_output.0, Some("/home".to_string()));
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
        Some("cd: /non/existent/directory: No such file or directory".to_string())
    );
}

#[test]
fn test_cd_previous() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let output = shell.execute("cd /home").expect("Failed to execute cd");
    assert_eq!(output.0, None);
    let pwd_output = shell.execute("pwd").expect("Failed to execute pwd");
    assert_eq!(pwd_output.0, Some("/home".to_string()));
    let output = shell.execute("cd /").expect("Failed to execute cd");
    assert_eq!(output.0, None);
    let pwd_output = shell.execute("pwd").expect("Failed to execute pwd");
    assert_eq!(pwd_output.0, Some("/".to_string()));
    let output = shell.execute("cd -").expect("Failed to execute cd");
    assert_eq!(output.0, None);
    let pwd_output = shell.execute("pwd").expect("Failed to execute pwd");
    assert_eq!(pwd_output.0, Some("/home".to_string()));
}

#[test]
fn test_parent_with_root() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let output = shell.execute("cd /").expect("Failed to execute cd");
    assert_eq!(output.0, None);
    let pwd_output = shell.execute("pwd").expect("Failed to execute pwd");
    assert_eq!(pwd_output.0, Some("/".to_string()));
    let output = shell.execute("cd ..").expect("Failed to execute cd");
    assert_eq!(output.0, None);
    let pwd_output = shell.execute("pwd").expect("Failed to execute pwd");
    assert_eq!(pwd_output.0, Some("/".to_string()));
}
