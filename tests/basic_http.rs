use assert_cmd::Command;
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{header, method, path};

fn rurl() -> Command {
    Command::cargo_bin("rurl").expect("binary not found")
}

#[tokio::test]
async fn test_get_request() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/get"))
        .respond_with(ResponseTemplate::new(200).set_body_string("hello world"))
        .mount(&server)
        .await;

    let url = format!("{}/get", server.uri());
    let output = rurl().arg(&url).output().expect("failed to run rurl");

    assert!(output.status.success(), "exit code should be 0");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "hello world"
    );
}

#[tokio::test]
async fn test_post_request() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/post"))
        .respond_with(ResponseTemplate::new(201).set_body_string("created"))
        .mount(&server)
        .await;

    let url = format!("{}/post", server.uri());
    let output = rurl()
        .args(["-X", "POST", "-d", "body=data", &url])
        .output()
        .expect("failed to run rurl");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "created"
    );
}

#[tokio::test]
async fn test_custom_header() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/headers"))
        .and(header("X-Custom", "TestValue"))
        .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
        .mount(&server)
        .await;

    let url = format!("{}/headers", server.uri());
    let output = rurl()
        .args(["-H", "X-Custom: TestValue", &url])
        .output()
        .expect("failed to run rurl");

    assert!(output.status.success());
}

#[tokio::test]
async fn test_json_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/json"))
        .and(header("content-type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"status":"ok"}"#))
        .mount(&server)
        .await;

    let url = format!("{}/json", server.uri());
    let output = rurl()
        .args(["--json", r#"{"key":"value"}"#, &url])
        .output()
        .expect("failed to run rurl");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("ok"));
}

#[tokio::test]
async fn test_silent_mode() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/silent"))
        .respond_with(ResponseTemplate::new(200).set_body_string("body"))
        .mount(&server)
        .await;

    let url = format!("{}/silent", server.uri());
    let output = rurl()
        .args(["-s", &url])
        .output()
        .expect("failed to run rurl");

    // Silent mode still outputs body to stdout
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "body");
    // stderr should be empty
    assert!(output.stderr.is_empty());
}

#[tokio::test]
async fn test_include_headers() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/include"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("x-test-header", "header-value")
                .set_body_string("body"),
        )
        .mount(&server)
        .await;

    let url = format!("{}/include", server.uri());
    let output = rurl()
        .args(["-i", &url])
        .output()
        .expect("failed to run rurl");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("HTTP/1.1 200"));
    assert!(stdout.contains("x-test-header: header-value"));
    assert!(stdout.contains("body"));
}

#[tokio::test]
async fn test_head_request() {
    let server = MockServer::start().await;
    Mock::given(method("HEAD"))
        .and(path("/head"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let url = format!("{}/head", server.uri());
    let output = rurl()
        .args(["-I", &url])
        .output()
        .expect("failed to run rurl");

    assert!(output.status.success());
    // No body for HEAD requests
    assert!(output.stdout.is_empty());
}

#[tokio::test]
async fn test_put_request() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/put"))
        .respond_with(ResponseTemplate::new(200).set_body_string("updated"))
        .mount(&server)
        .await;

    let url = format!("{}/put", server.uri());
    let output = rurl()
        .args(["-X", "PUT", "-d", "data", &url])
        .output()
        .expect("failed to run rurl");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "updated");
}

#[tokio::test]
async fn test_delete_request() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/delete"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let url = format!("{}/delete", server.uri());
    let output = rurl()
        .args(["-X", "DELETE", &url])
        .output()
        .expect("failed to run rurl");

    assert!(output.status.success());
}

#[tokio::test]
async fn test_write_out_http_code() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/status"))
        .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
        .mount(&server)
        .await;

    let url = format!("{}/status", server.uri());
    let output = rurl()
        .args(["-s", "-w", "%{http_code}", "-o", "/dev/null", &url])
        .output()
        .expect("failed to run rurl");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "200");
}

#[tokio::test]
async fn test_output_to_file() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/file"))
        .respond_with(ResponseTemplate::new(200).set_body_string("file contents"))
        .mount(&server)
        .await;

    let tmp = tempfile::NamedTempFile::new().unwrap();
    let tmp_path = tmp.path().to_str().unwrap().to_string();

    let url = format!("{}/file", server.uri());
    let output = rurl()
        .args(["-o", &tmp_path, &url])
        .output()
        .expect("failed to run rurl");

    assert!(output.status.success());
    let content = std::fs::read_to_string(&tmp_path).unwrap();
    assert_eq!(content.trim(), "file contents");
}
