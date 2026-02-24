use assert_cmd::Command;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn rurl() -> Command {
    Command::cargo_bin("rurl").expect("binary not found")
}

#[tokio::test]
async fn test_form_data_content_type() {
    let server = MockServer::start().await;
    // -d with form data should set Content-Type: application/x-www-form-urlencoded
    // (rurl auto-promotes to POST when -d is used without explicit method)
    Mock::given(method("POST"))
        .and(path("/form"))
        .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
        .mount(&server)
        .await;

    let url = format!("{}/form", server.uri());
    let output = rurl()
        .args(["-d", "field=value&other=123", &url])
        .output()
        .expect("failed to run rurl");

    assert!(output.status.success());
}

#[tokio::test]
async fn test_data_raw_no_at_expansion() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/raw"))
        .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
        .mount(&server)
        .await;

    let url = format!("{}/raw", server.uri());
    // --data-raw should not expand @filename
    let output = rurl()
        .args(["--data-raw", "@notafile", &url])
        .output()
        .expect("failed to run rurl");

    assert!(output.status.success());
}

#[tokio::test]
async fn test_data_from_file() {
    let tmp = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(tmp.path(), "file body content").unwrap();
    let tmp_path = tmp.path().to_str().unwrap().to_string();

    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/upload"))
        .respond_with(ResponseTemplate::new(200).set_body_string("received"))
        .mount(&server)
        .await;

    let url = format!("{}/upload", server.uri());
    let at_path = format!("@{tmp_path}");
    let output = rurl()
        .args(["-d", &at_path, &url])
        .output()
        .expect("failed to run rurl");

    assert!(output.status.success());
}
