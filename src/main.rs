use actix_web::{HttpServer};

use actix_testing_example::make_app;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || make_app())
        .bind(("0.0.0.0", 12345))?
        .run()
        .await
}
