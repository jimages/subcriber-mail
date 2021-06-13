use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use subcriber_mail::configuration::get_configuration;

#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscribe_return_a_200_for_valid_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let mut conection = PgConnection::connect(&connection_string)
        .await
        .expect("Fail to connect postgres");

    let body = "name=le%20guin&email=foo@gamil.com";

    let response = client
        .post(format!("{}/subscriptions", app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut conection)
        .await
        .expect("Fail to fetch saved subscription.");

    assert_eq!(saved.email, "foo@gamil.com");
    assert_eq!(saved.name, "le guin");
}

#[actix_rt::test]
async fn subscribe_return_a_400_when_data_is_missing() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=zachary%20wang", "missing the email"),
        ("email=jimages123%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "API did not fail with 400 bad request when payload was {}.",
            error
        );
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let port = listener.local_addr().unwrap().port();
    let server = subcriber_mail::run(listener).expect("fail to bind address");
    let _ = actix_rt::spawn(async {
        server.await.expect("test server");
    });
    format!("http://127.0.0.1:{}", port)
}
