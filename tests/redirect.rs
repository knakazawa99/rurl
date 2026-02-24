use assert_cmd::Command;
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path};

fn rurl() -> Command {
    Command::cargo_bin("rurl").expect("binary not found")
}

#[tokio::test]
async fn test_redirect_not_followed_by_default() {
    let server = MockServer::start().await;
    let target_url = format!("{}/final", server.uri());

    Mock::given(method("GET"))
        .and(path("/redirect"))
        .respond_with(
            ResponseTemplate::new(302)
                .insert_header("location", &target_url as &str),
        )
        .mount(&server)
        .await;

    let url = format!("{}/redirect", server.uri());
    // Without -L, should NOT follow redirect and return 302
    let output = rurl()
        .args(["-s", "-w", "%{http_code}", "-o", "/dev/null", &url])
        .output()
        .expect("failed to run rurl");

    assert!(output.status.success()); // exit 0 for HTTP responses
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "302");
}

#[tokio::test]
async fn test_redirect_followed_with_l_flag() {
    let server = MockServer::start().await;
    let target_url = format!("{}/final", server.uri());

    Mock::given(method("GET"))
        .and(path("/redirect"))
        .respond_with(
            ResponseTemplate::new(302)
                .insert_header("location", &target_url as &str),
        )
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/final"))
        .respond_with(ResponseTemplate::new(200).set_body_string("final destination"))
        .mount(&server)
        .await;

    let url = format!("{}/redirect", server.uri());
    let output = rurl()
        .args(["-L", &url])
        .output()
        .expect("failed to run rurl");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "final destination"
    );
}
