use shelljougahara::Shell;

#[test]
fn test_mkdir() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let mkdir = shell
        .execute("mkdir test")
        .expect("Failed to execute mkdir");
    let cd = shell.execute("cd test").expect("Failed to execute cd");
    assert_eq!(mkdir.0, None);
    assert_eq!(cd.0, None);
    let pwd = shell.execute("pwd").expect("Failed to execute pwd");
    assert_eq!(pwd.0, Some(format!("/home/{username}/test")));
}

#[test]
fn test_mkdir_multiple() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let mkdir = shell
        .execute("mkdir test1 test2")
        .expect("Failed to execute mkdir");
    assert_eq!(mkdir.0, None);
    let cd1 = shell
        .execute(format!("cd /home/{username}/test1").as_str())
        .expect("Failed to execute cd");
    assert_eq!(cd1.0, None);
    let pwd1 = shell.execute("pwd").expect("Failed to execute pwd");
    assert_eq!(pwd1.0, Some(format!("/home/{username}/test1")));
    let cd2 = shell
        .execute(format!("cd /home/{username}/test2").as_str())
        .expect("Failed to execute cd");
    assert_eq!(cd2.0, None);
    let pwd2 = shell.execute("pwd").expect("Failed to execute pwd");
    assert_eq!(pwd2.0, Some(format!("/home/{username}/test2")));
}

#[test]
fn test_mkdir_existing_directory() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let mkdir = shell
        .execute("mkdir test")
        .expect("Failed to execute mkdir");
    assert_eq!(mkdir.0, None);
    let mkdir2 = shell
        .execute("mkdir test")
        .expect("Failed to execute mkdir");
    assert_eq!(
        mkdir2.0,
        Some("mkdir: cannot create directory 'test': File exists".to_string())
    );
}

#[test]
fn test_mkdir_no_such_file_or_directory() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let mkdir = shell
        .execute("mkdir test/test2")
        .expect("Failed to execute mkdir");
    assert_eq!(
        mkdir.0,
        Some("mkdir: cannot create directory 'test/test2': No such file or directory".to_string())
    );
}

#[test]
fn test_mkdir_long_path() {
    let username = "test";
    let mut shell = Shell::new_with_user(username);
    let initial_mkdir = shell
        .execute("mkdir test")
        .expect("Failed to execute mkdir");
    assert_eq!(initial_mkdir.0, None);
    let mkdir = shell
        .execute("mkdir test/test2")
        .expect("Failed to execute mkdir");
    assert_eq!(mkdir.0, None);
    let cd = shell
        .execute("cd test/test2")
        .expect("Failed to execute cd");
    assert_eq!(cd.0, None);
    let pwd = shell.execute("pwd").expect("Failed to execute pwd");
    assert_eq!(pwd.0, Some(format!("/home/{username}/test/test2")));
}
