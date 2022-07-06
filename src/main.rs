use actix_web::{App, HttpServer};

use actix_testing_example::register;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().configure(register))
        .bind(("0.0.0.0", 12345))?
        .run()
        .await
}
