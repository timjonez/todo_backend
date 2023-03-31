use actix_web::{App, HttpServer};
use todo_backend::accounts::{create_user, user_login};
use todo_backend::base::Auth;
use todo_backend::todos::{get_list, get_todo};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Auth)
            .service(create_user)
            .service(user_login)
            .service(get_list)
            .service(get_todo)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
