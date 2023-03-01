use crate::db::setup_db;
use actix_web::{get, post, web, HttpResponse, Responder};
use surrealdb::sql::thing;

#[get("/todos")]
pub async fn get_list() -> impl Responder {
    let (db, session) = setup_db().await;

    let query = "select * from todo";
    let res = db.execute(query, &session, None, false).await.unwrap();

    HttpResponse::Ok().json(res.first().unwrap().output().unwrap())
}

#[get("/todos/{id}")]
pub async fn get_todo(path: web::Path<String>) -> impl Responder {
    let (db, session) = setup_db().await;

    let query = "select * from todo where id = $id ;";
    let id = thing(format!("todo:{}", path.into_inner()).as_str()).unwrap();
    let vars = [("id".into(), id.into())].into();

    let res = db
        .execute(&query, &session, Some(vars), false)
        .await
        .unwrap();

    let result = res.first().unwrap().output().unwrap();

    HttpResponse::Ok().json(result)
}

#[post("/todos/")]
pub async fn create_todo(path: web::Path<String>) -> impl Responder {
    let (db, session) = setup_db().await;

    let query = "CREATE todo CONTENT $data";
    let id = thing(format!("todo:{}", path.into_inner()).as_str()).unwrap();
    let vars = [("id".into(), id.into())].into();

    let res = db
        .execute(&query, &session, Some(vars), false)
        .await
        .unwrap();

    let result = res.first().unwrap().output().unwrap();

    HttpResponse::Ok().json(result)
}
