use crate::accounts::User;
use chrono::{DateTime, NaiveDate, Utc};

pub struct Todo {
    pub user: User,
    pub title: String,
    pub description: String,
    pub priority: i8,
    pub complete: bool,
    pub date_due: NaiveDate,
    pub created: DateTime<Utc>,
}
