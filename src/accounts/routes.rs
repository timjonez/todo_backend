use actix_web::{get, post, web, HttpResponse, Responder};
use crate::db::setup_db;
use crate::accounts::models::User;
use surrealdb::sql::thing;
use email_address::EmailAddress;
use std::collections::HashMap;


#[derive(serde::Deserialize)]
pub struct CreateUserPayload {
    pub email: EmailAddress,
    pub password: String,
    pub name: String,
}


#[post("/users/")]
pub async fn create_user(payload: web::Json<CreateUserPayload>) -> impl Responder {
    let (db, session) = setup_db().await;
    let user = User::new(payload.email.to_string(), payload.password.clone(), payload.name.clone()).unwrap();

    let query = "CREATE user SET name = $name, email = $email, password = $password";
    let vars = [
        ("name".into(), user.name.into()),
        ("email".into(), user.email.to_string().into()),
        ("password".into(), user.password.password().into())
    ].into();

    let res = db.execute(&query, &session, Some(vars), false)
        .await
        .unwrap();

    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
pub struct LoginPayload {
    pub email: EmailAddress,
    pub password: String,
}

#[post("/login/")]
pub async fn user_login(payload: web::Json<LoginPayload>) -> impl Responder {
    let (db, session) = setup_db().await;

    let query = "SELECT * FROM user where email = $email";
    let vars = [("email".into(), payload.email.to_string().into())].into();
    let res = match db.execute(query, &session, Some(vars), false).await {
        Err(..) => { return HttpResponse::BadRequest().finish()},
        Ok(r) => r 
    };
    let user: User = match res.len() {
        1 => {
            User::from(res.first().expect("err1").result.as_ref().expect("err2").clone())
        },
        _ => { return HttpResponse::BadRequest().json(HashMap::from([("error", "Invalid credentials")])) }
    };

    match &user.password.check_password(&payload.password) {
        true => { HttpResponse::Ok().json(user) },
        false => { HttpResponse::Unauthorized().finish() }
    }
}