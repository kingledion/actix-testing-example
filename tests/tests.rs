use actix_testing_example::User;

#[tokio::test]
async fn test_signup() {

    let payload_str = r#"{"email": "me@gmail.com","password": "verysecure"}"#;
    println!("string argument: {:?}", payload_str);


    let mut u = User{
        email: "me@gmail.com".to_string(),
        password: "verysecure".to_string()
    };
    let payload_ser = serde_json::to_string(&u).unwrap();
    println!("string argument: {:?}", payload_ser);

    println!("payload argument: {:?}", arg_payload);


    let example: crypto::UserClear = serde_json::from_str(&arg_payload).unwrap();
    println!("example deserialization: {:?}", example);
    
    let exp_status = StatusCode::OK;
    let exp_id = crypto::make_id("kingledion@gmail.com");
    
    let resp = test_signup_app(arg_payload_raw.to_string()).await;

    let resp_status =  resp.status();

    let resp_body = test::read_body(resp).await;
    assert_eq!(
        resp_status, 
        exp_status, 
        "Got unexpected status {}, expected {}, result body: {:?}", 
        resp_status, exp_status, resp_body
    );


    let id: handler::UserId = 
        serde_json::from_str(std::str::from_utf8(&resp_body).unwrap()).unwrap();
    assert_eq!(id.id, exp_id);

}

async fn test_signup_app(payload: String) -> ServiceResponse {

    let test_repo = TestRepo::new(&CONFIG.repository).await;
    let repo_data = web::Data::new(test_repo.repo.clone());

    let crypto = web::Data::new(
        crypto::Encrypter::new(CONFIG.crypto.secret_key.as_bytes())
    );

    println!("About to initialize app for signup test");

    let app = test::init_service(
        App::new()
            .app_data(repo_data)
            .app_data(crypto.clone())
            .route("/users/signup/", web::post().to(handler::signup))
    ).await;

    let req = test::TestRequest::post()
        .uri("/users/signup/")
        .set_payload(payload)
        .to_request();

    println!("new request: {:?}", req);

    //let r = test::call_and_read_body_json(&app, req).await;
    test::call_service(&app, req).await

}