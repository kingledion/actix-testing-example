use actix_web::body::MessageBody;
use actix_web::{App, web, HttpResponse, Error};
use actix_web::dev::{ServiceRequest, ServiceResponse};

use serde::{Serialize, Deserialize};

pub fn make_app() -> App<
    impl actix_web::dev::ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    App::new().service(
        web::resource("/signup").route(web::post().to(signup)),
    )
}

pub async fn signup(arg_user_bytes: web::Bytes) -> HttpResponse {
    println!("raw bytes at handler: {:?}", arg_user_bytes);

    let user: User = serde_json::from_str(std::str::from_utf8(&arg_user_bytes).unwrap()).unwrap();

    println!("deserialized to struct: {:?}", user);

    HttpResponse::Ok().json(user)
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct User {
    pub email: String,
    pub password: String, 
}
