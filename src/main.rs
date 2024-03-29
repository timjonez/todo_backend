use actix_web::{App, HttpServer};
use todo_backend::accounts::{create_user, user_login, get_current_user};
use todo_backend::base::Auth;
use todo_backend::todos::{get_list, get_todo, create_todo};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            // .wrap(Auth)
            .service(create_user)
            .service(user_login)
            .service(create_todo)
            .service(get_list)
            .service(get_todo)
            .service(get_current_user)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
