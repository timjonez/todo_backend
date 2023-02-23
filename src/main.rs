use actix_web::{App, HttpServer};
use todo_backend::todos::{get_list, get_todo};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_list)
            .service(get_todo)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
