use actix_web::dev::Server;
use reqwest::{Client, Response};
use zero2prod::run;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind");
    let port: u16 = listener.local_addr().unwrap().port();
    let server: Server = run(listener).expect("Failed to start server");
    tokio::spawn(server);

    let client: Client = reqwest::Client::new();

    let response: Response = client
        .get(&format!("http://127.0.0.1:{}/health_check", port))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}


#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to start server");
    tokio::spawn(server);

    let client = reqwest::Client::new();
    let body = "name=Sohan&email=sohan%40example.com"; // %40 is '@'

    // Act
    let response = client
        .post(&format!("http://127.0.0.1:{}/subscriptions", port))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
}
