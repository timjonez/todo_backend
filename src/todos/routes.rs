use actix_web::{get, HttpResponse, Responder};
use crate::db::setup_db;

#[get("/")]
pub async fn get_list() -> impl Responder {
    let (db, session) = setup_db().await;

    let query = "select * from todo";
    let res = db.execute(query, &session, None, false)
        .await
        .unwrap();

    HttpResponse::Ok().json(res.first().unwrap().output().unwrap())
}
