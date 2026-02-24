use assert_cmd::Command;
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path};

fn rurl() -> Command {
    Command::cargo_bin("rurl").expect("binary not found")
}

#[tokio::test]
async fn test_verbose_output_to_stderr() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/verbose"))
        .respond_with(ResponseTemplate::new(200).set_body_string("body"))
        .mount(&server)
        .await;

    let url = format!("{}/verbose", server.uri());
    let output = rurl()
        .args(["-v", &url])
        .output()
        .expect("failed to run rurl");

    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Verbose output goes to stderr
    assert!(stderr.contains("GET /verbose"), "stderr: {stderr}");
    assert!(stderr.contains("HTTP/1.1 200"), "stderr: {stderr}");
}

#[tokio::test]
async fn test_write_out_url_effective() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/url"))
        .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
        .mount(&server)
        .await;

    let url = format!("{}/url", server.uri());
    let output = rurl()
        .args(["-s", "-w", "%{url_effective}\\n", "-o", "/dev/null", &url])
        .output()
        .expect("failed to run rurl");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("/url"), "stdout: {stdout}");
}

#[tokio::test]
async fn test_remote_name_flag() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/testfile.txt"))
        .respond_with(ResponseTemplate::new(200).set_body_string("remote content"))
        .mount(&server)
        .await;

    let url = format!("{}/testfile.txt", server.uri());

    // Run in a temp directory
    let tmp_dir = tempfile::tempdir().unwrap();
    let output = rurl()
        .args(["-O", &url])
        .current_dir(tmp_dir.path())
        .output()
        .expect("failed to run rurl");

    assert!(output.status.success());
    let saved = tmp_dir.path().join("testfile.txt");
    assert!(saved.exists(), "file should be saved as testfile.txt");
    let content = std::fs::read_to_string(saved).unwrap();
    assert_eq!(content.trim(), "remote content");
}
