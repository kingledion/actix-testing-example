use actix_web::{web, HttpResponse};

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("users").service(web::resource("/signup").route(web::post().to(signup))),
    );
}

pub async fn signup(arg_user_bytes: web::Bytes) -> HttpResponse {
    println!("raw bytes: {:?}", arg_user_bytes);

    let user: User = serde_json::from_str(std::str::from_utf8(&arg_user_bytes).unwrap()).unwrap();

    println!("unwrapped bytes: {:?}", user);

    HttpResponse::Ok().json(user)
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct User {
    pub email: String,
    pub password: String, // un-encrypted
}
