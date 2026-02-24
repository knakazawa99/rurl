use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

/// Convenience wrapper: spin up a WireMock server and return the base URL
pub async fn start() -> (MockServer, String) {
    let server = MockServer::start().await;
    let url = server.uri();
    (server, url)
}

/// Mount a simple GET handler that returns 200 + body
pub async fn mount_get(server: &MockServer, path_str: &str, body: &str) {
    Mock::given(method("GET"))
        .and(path(path_str))
        .respond_with(ResponseTemplate::new(200).set_body_string(body))
        .mount(server)
        .await;
}

/// Mount a simple POST handler that returns 201 + body
pub async fn mount_post(server: &MockServer, path_str: &str, response_body: &str) {
    Mock::given(method("POST"))
        .and(path(path_str))
        .respond_with(ResponseTemplate::new(201).set_body_string(response_body))
        .mount(server)
        .await;
}
