use todo_backend::db::setup;
use actix_web::{App, HttpServer};
use todo_backend::todos::get_list;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = setup().await;

    HttpServer::new(|| {
        App::new()
            .service(get_list)
            .app_data(db.clone())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
