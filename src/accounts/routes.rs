use crate::accounts::models::User;
use crate::base::setup_db;
use crate::base::{check_token, get_token, Arg, Lookup, Model};
use actix_web::{get, post, web, HttpResponse, Responder};
use email_address::EmailAddress;
use serde_json::json;
use std::collections::{HashMap, BTreeMap};
use surrealdb::sql::Value;


#[derive(serde::Deserialize)]
pub struct CreateUserPayload {
    pub email: EmailAddress,
    pub password: String,
    pub name: String,
}


#[get("/user/")]
pub async fn get_current_user(user: web::ReqData<Option<String>>,) -> impl Responder {
    let arg = Arg {
        field: "id".to_string(),
        value: user.as_ref().unwrap().clone(),
        lookup: Lookup::Exact,
    };
    let user = User::get(arg).await.unwrap();
    HttpResponse::Ok().json(user)
}

#[post("/users/")]
pub async fn create_user(payload: web::Json<CreateUserPayload>) -> impl Responder {
    let mut data: BTreeMap<String, Value> = BTreeMap::new();
    data.insert("email".to_string(), payload.email.clone().to_string().into());
    data.insert("password".to_string(), payload.password.clone().into());
    data.insert("name".to_string(), payload.name.to_string().into());

    println!("1111 {:?}", data);
    let r = User::create(data).await;
    HttpResponse::Ok().body("This thing")

    // let (db, session) = setup_db().await;
    // let user = User::new(
    //     payload.email.to_string(),
    //     payload.password.clone(),
    //     payload.name.clone(),
    // )
    // .unwrap();

    // let query = "CREATE user SET name = $name, email = $email, password = $password";
    // let vars = [
    //     ("name".into(), user.name.into()),
    //     ("email".into(), user.email.to_string().into()),
    //     ("password".into(), user.password.password().into()),
    // ]
    // .into();

    // let _res = db
    //     .execute(&query, &session, Some(vars), false)
    //     .await
    //     .unwrap();

    // HttpResponse::Ok().finish()
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
        Err(..) => return HttpResponse::BadRequest().finish(),
        Ok(r) => r,
    };
    let user: User = match res.len() {
        1 => User::from(
            res.first()
                .expect("err1")
                .result
                .as_ref()
                .expect("err2")
                .clone(),
        ),
        _ => {
            return HttpResponse::BadRequest()
                .json(HashMap::from([("error", "Invalid credentials")]))
        }
    };
    println!("aaa> {}", payload.password);

    match &user.password.check_password(&payload.password) {
        false => HttpResponse::Unauthorized().finish(),
        true => {
            let token = get_token(user.id, user.email);
            println!("> {:?}", token);
            let payload = check_token(token.clone()).unwrap();
            println!(">>> {:?}", payload);
            HttpResponse::Ok().json(json!({ "token": token }))
        }
    }
}
