use actix_web::{get, web, HttpResponse, Responder};
use crate::db::setup_db;

#[get("/todos")]
pub async fn get_list() -> impl Responder {
    let (db, session) = setup_db().await;

    let query = "select * from todo";
    let res = db.execute(query, &session, None, false)
        .await
        .unwrap();

    HttpResponse::Ok().json(res.first().unwrap().output().unwrap())
}

#[get("/todos/{id}")]
pub async fn get_todo(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();

    let (db, session) = setup_db().await;

    let query = format!("select * from todo where id = 'todo:{}'", id);
    let res = db.execute(&query, &session, None, false)
        .await
        .unwrap();

    let result = res.first()
        .unwrap()
        .output()
        .unwrap();

    HttpResponse::Ok().json(result)
}
