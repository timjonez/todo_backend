use crate::accounts::User;
use crate::base::{Arg, Model};
use chrono::{DateTime, NaiveDate, Utc};
use async_trait::async_trait;
use std::collections::BTreeMap;
use surrealdb::sql::Value;

pub struct Todo {
    pub user: User,
    pub title: String,
    pub description: String,
    pub priority: i8,
    pub complete: bool,
    pub date_due: NaiveDate,
    pub created: DateTime<Utc>,
}

#[async_trait]
impl Model<Todo> for Todo {
    async fn get(args: Arg) -> Result<Todo, String> {
        let db = Self::db().await;
        let session = Self::session().await;
        Err("Not implemented".to_string())
    }

    async fn create(data: BTreeMap<String, Value>) -> Result<Todo, String> {
        println!("aaaaaaaa");
        let db = Self::db().await;
        let session = Self::session().await;

        let query = "CREATE todo CONTENT $data";
        let vars = [("data".into(), data.into())].into();
        println!("bbbbbbbbb");
        let res = db
            .execute(&query, &session, Some(vars), false)
            .await
            .unwrap();
        
        Err("Not implemented".to_string())
    }
}
