use assert_cmd::Command;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn rurl() -> Command {
    Command::cargo_bin("rurl").expect("binary not found")
}

#[tokio::test]
async fn test_basic_auth() {
    let server = MockServer::start().await;
    // Basic auth for user:pass → "dXNlcjpwYXNz" in base64
    Mock::given(method("GET"))
        .and(path("/auth"))
        .and(header("authorization", "Basic dXNlcjpwYXNz"))
        .respond_with(ResponseTemplate::new(200).set_body_string("authenticated"))
        .mount(&server)
        .await;

    let url = format!("{}/auth", server.uri());
    let output = rurl()
        .args(["-u", "user:pass", &url])
        .output()
        .expect("failed to run rurl");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "authenticated"
    );
}

#[tokio::test]
async fn test_user_agent() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/ua"))
        .and(header("user-agent", "MyCustomAgent/1.0"))
        .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
        .mount(&server)
        .await;

    let url = format!("{}/ua", server.uri());
    let output = rurl()
        .args(["-A", "MyCustomAgent/1.0", &url])
        .output()
        .expect("failed to run rurl");

    assert!(output.status.success());
}

#[tokio::test]
async fn test_referer_header() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/referer"))
        .and(header("referer", "https://example.com"))
        .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
        .mount(&server)
        .await;

    let url = format!("{}/referer", server.uri());
    let output = rurl()
        .args(["-e", "https://example.com", &url])
        .output()
        .expect("failed to run rurl");

    assert!(output.status.success());
}
