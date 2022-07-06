use actix_testing_example::{make_app, User};

use actix_web::test;

#[tokio::test]
async fn payload_reference() {

    let payload = r#"{"email":"me@gmail.com","password": "verysecure"}"#;
    println!("string payload: {:?}", payload);

    let app = test::init_service(
        make_app()
    ).await;

    let req = test::TestRequest::post()
        .uri("/users/signup/")
        .set_payload(payload)
        .to_request();

    let resp: User = test::call_and_read_body_json(&app, req).await;

    println!("got response: {:?}", resp);
    assert!(false);

}

#[tokio::test]
async fn set_json() {

    let payload = r#"{"email":"me@gmail.com","password": "verysecure"}"#.to_string();
    println!("string payload: {:?}", payload);

    let app = test::init_service(
        make_app()
    ).await;

    let req = test::TestRequest::post()
        .uri("/signup")
        .set_json(payload)
        .to_request();

    let resp: User = test::call_and_read_body_json(&app, req).await;

    println!("got response: {:?}", resp);
    assert!(false);

}

#[tokio::test]
async fn success() {

    let payload = r#"{"email":"me@gmail.com","password": "verysecure"}"#.to_string();
    println!("string argument: {:?}", payload);

    let app = test::init_service(
        make_app()
    ).await;

    let req = test::TestRequest::post()
        .uri("/signup")
        .set_payload(payload)
        .to_request();

    let resp: User = test::call_and_read_body_json(&app, req).await;

    println!("got response: {:?}", resp);
    assert!(false);

}