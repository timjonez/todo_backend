use actix_web::{get, web, HttpResponse, Responder};
use crate::db::App;

#[get("/")]
pub async fn get_list(db: web::Data<App>) -> impl Responder {
    let query = "select * from todo";
    let res = &db.db.execute(query, &db.session, None, false)
        .await
        .unwrap();


    HttpResponse::Ok().json(res)
}
