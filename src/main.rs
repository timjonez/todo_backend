use todo_backend::db::setup;
use actix_web::{App, HttpServer};
use todo_backend::todos::hello;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _app = setup().await;

    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
