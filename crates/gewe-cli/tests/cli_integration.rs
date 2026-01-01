use std::process::Command;

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(["run", "-p", "gewe-cli", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("GeWe CLI"));
    assert!(stdout.contains("Usage:"));
}

#[test]
fn test_cli_version() {
    let output = Command::new("cargo")
        .args(["run", "-p", "gewe-cli", "--", "--version"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("gewe"));
}

#[test]
fn test_cli_verbose_flag() {
    let output = Command::new("cargo")
        .args(["run", "-p", "gewe-cli", "--", "-v", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}

#[test]
fn test_cli_double_verbose_flag() {
    let output = Command::new("cargo")
        .args(["run", "-p", "gewe-cli", "--", "-vv", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}

#[test]
fn test_cli_command_list() {
    let output = Command::new("cargo")
        .args(["run", "-p", "gewe-cli", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // 验证一些关键命令存在
    assert!(stdout.contains("send-text"));
    assert!(stdout.contains("send-image"));
    assert!(stdout.contains("send-video"));
    assert!(stdout.contains("send-file"));
    assert!(stdout.contains("get-login-qr"));
    assert!(stdout.contains("check-login"));
    assert!(stdout.contains("config"));
}

#[test]
fn test_cli_invalid_command() {
    let output = Command::new("cargo")
        .args(["run", "-p", "gewe-cli", "--", "invalid-command"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("error") || stderr.contains("unrecognized"));
}

#[test]
fn test_cli_config_flag() {
    let output = Command::new("cargo")
        .args([
            "run",
            "-p",
            "gewe-cli",
            "--",
            "--config",
            "/tmp/test_config.toml",
            "--help",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}
