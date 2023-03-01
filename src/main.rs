use actix_web::{App, HttpServer};
use todo_backend::accounts::User;
use todo_backend::todos::{get_list, get_todo};
use todo_backend::accounts::{create_user, user_login};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_list).service(get_todo).service(create_user).service(user_login))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
