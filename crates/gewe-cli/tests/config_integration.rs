use std::fs;
use tempfile::TempDir;

#[test]
fn test_config_command_set_token() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    let output = std::process::Command::new("cargo")
        .args([
            "run",
            "-p",
            "gewe-cli",
            "--",
            "--config",
            config_path.to_str().unwrap(),
            "config",
            "--token",
            "test_token_123",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(config_path.exists());

    let contents = fs::read_to_string(&config_path).unwrap();
    assert!(contents.contains("test_token_123"));
}

#[test]
fn test_config_command_set_base_url() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    let output = std::process::Command::new("cargo")
        .args([
            "run",
            "-p",
            "gewe-cli",
            "--",
            "--config",
            config_path.to_str().unwrap(),
            "config",
            "--base-url",
            "http://localhost:8080",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(config_path.exists());

    let contents = fs::read_to_string(&config_path).unwrap();
    assert!(contents.contains("http://localhost:8080"));
}

#[test]
fn test_config_command_set_multiple_fields() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    let output = std::process::Command::new("cargo")
        .args([
            "run",
            "-p",
            "gewe-cli",
            "--",
            "--config",
            config_path.to_str().unwrap(),
            "config",
            "--token",
            "my_token",
            "--base-url",
            "http://api.example.com",
            "--app-id",
            "app_123",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(config_path.exists());

    let contents = fs::read_to_string(&config_path).unwrap();
    assert!(contents.contains("my_token"));
    assert!(contents.contains("http://api.example.com"));
    assert!(contents.contains("app_123"));
}

#[test]
fn test_config_command_view_empty() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    let output = std::process::Command::new("cargo")
        .args([
            "run",
            "-p",
            "gewe-cli",
            "--",
            "--config",
            config_path.to_str().unwrap(),
            "config",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Empty config should show default TOML structure
    assert!(stdout.contains("bots = []") || stdout.is_empty() || !stdout.contains("token"));
}

#[test]
fn test_config_creates_parent_directory() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir
        .path()
        .join("nested")
        .join("path")
        .join("config.toml");

    let output = std::process::Command::new("cargo")
        .args([
            "run",
            "-p",
            "gewe-cli",
            "--",
            "--config",
            config_path.to_str().unwrap(),
            "config",
            "--token",
            "test",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(config_path.exists());
    assert!(config_path.parent().unwrap().exists());
}
