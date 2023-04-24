use crate::base::setup_db;
use crate::base::Model;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, DateTime, Utc};
use surrealdb::sql::{thing, Value};
use crate::todos::models::Todo;
use std::collections::{BTreeMap, HashMap};


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

#[derive(Deserialize, Debug)]
pub struct CreateTodo {
    pub title: String,
    pub description: String,
    pub priority: i8,
    pub date_due: NaiveDate,
}

#[post("/todos/")]
pub async fn create_todo(payload: web::Json<CreateTodo>) -> impl Responder {
    println!("0000 {:?}", payload);
    // let date = match NaiveDate::parse_from_str(payload.date_due) {
    //     Ok(d) => d,
    //     Err(e) => return HttpResponse::BadRequest().finish()
    // }
    let mut data: BTreeMap<String, Value> = BTreeMap::new();
    data.insert("title".to_string(), payload.title.clone().into());
    data.insert("description".to_string(), payload.description.clone().into());
    data.insert("priority".to_string(), payload.priority.to_string().into());
    data.insert("date_due".to_string(), Utc::now().date_naive().format("%Y-%m-%d").to_string().into());
    // map.append("title".to_string());
    // let data: BTreeMap<String, Value> = [
    //     ("title".to_string(), payload.title.into()),
    //     ("description".to_string(), payload.description.into()),
    //     ("priority".to_string(), payload.priority.to_string().into()),
    //     ("date_due".to_string(), Utc::now().date_naive().format("%Y-%m-%d").to_string()),
    // ].into();
    Todo::create(data.into()).await;

    HttpResponse::Ok().finish()
    // let vars = [("id".into(), id.into())].into();

    // let res = db
    //     .execute(&query, &session, Some(vars), false)
    //     .await
    //     .unwrap();

    // let result = res.first().unwrap().output().unwrap();

    // HttpResponse::Ok().json(result)
}
