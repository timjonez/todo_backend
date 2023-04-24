use crate::base::{get_database, get_session};
use async_trait::async_trait;
use surrealdb::{Datastore, Session};
use surrealdb::sql::Value;
use std::collections::BTreeMap;

#[async_trait]
pub trait Model<T> {
    async fn db() -> Datastore {
        get_database().await
    }
    async fn session() -> Session {
        get_session().await
    }
    async fn get(args: Arg) -> Result<T, String>;
    // async fn get_all(args: Arg) -> T;
    async fn create(data: BTreeMap<String, Value>) -> Result<T, String>;
    // async fn update(&self) -> T;
    // async fn delete(&self);
}

#[derive(Debug)]
pub enum Lookup {
    Exact,
    IExact,
    Contains,
    IContains,
}

pub struct Arg {
    pub field: String,
    pub value: String,
    pub lookup: Lookup,
}

impl Arg {
    pub fn format_query(&self) -> String {
        match self.lookup {
            Lookup::Exact => {
                return format!("{} = '{}'", self.field, self.value)
            }
            _ => {
                return format!("Not implemented")
            }
        }
    }
}
