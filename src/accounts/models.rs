use crate::base::{Arg, Model};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use email_address::EmailAddress;
use passwords::{analyzer, hasher};
use std::str::FromStr;
use surrealdb::sql::Value;
use serde::{Serialize, Deserialize};

 #[derive(Default)]
pub struct Password {
    hashed_password: String,
}

impl Password {
    pub fn new(password: String) -> Password {
        Password {
            hashed_password: password,
        }
    }

    pub fn create_new(password: String) -> Result<Password, Vec<String>> {
        let mut errors = vec![];
        let analyzed = analyzer::analyze(password.clone());
        if &analyzed.length() < &8 {
            errors.push("Password must be at least 8 characters long".to_string());
        }
        if &analyzed.uppercase_letters_count() < &1 {
            errors.push("Password must contain at least one uppercase character".to_string());
        }

        if !errors.is_empty() {
            return Err(errors);
        }
        let hashed_password = Self::hash_password(&password);
        Ok(Password { hashed_password })
    }

    pub fn check_password(&self, password: &String) -> bool {
        println!("check password now");
        unsafe { hasher::identify_bcrypt_format(&password.as_str(), &self.hashed_password) }
    }

    fn hash_password(password: &String) -> String {
        let salt = hasher::gen_salt();
        hasher::bcrypt_format(10, &salt, &password).expect("unable to hash password")
    }

    pub fn password(self) -> String {
        self.hashed_password
    }
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: EmailAddress,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub password: Password,
    pub name: String,
    pub is_admin: bool,
    pub is_active: bool,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}

impl User {
    pub fn new(email: String, password: String, name: String) -> Result<User, Vec<String>> {
        let valid_email = EmailAddress::from_str(email.as_str()).expect("bad email");
        let hashed_password = Password::new(password);

        let user = User {
            id: "".to_string(),
            email: valid_email,
            password: hashed_password,
            name,
            is_admin: false,
            is_active: true,
            created: Utc::now(),
            modified: Utc::now(),
        };

        Ok(user)
    }

    pub fn into_btree_map(self) -> std::collections::BTreeMap<String, surrealdb::sql::Value> {
        [
            ("email".to_string(), self.email.to_string().into()),
            ("name".to_string(), self.name.into()),
        ]
        .into()
    }
}

#[async_trait]
impl Model<User> for User {
    async fn get(args: Arg) -> Result<User, String> {
        let db = Self::db().await;
        let session = Self::session().await;

        let where_clause = args.format_query();
        let query = "SELECT * FROM user where $where_clause;";

        let vars = [("where_clause".into(), where_clause.into())].into();
        let res = match db.execute(query, &session, Some(vars), false).await {
            Err(e) => return Err(e.to_string()),
            Ok(r) => r,
        };

        match res.len() {
            0 => return Err("User not found".to_string()),
            1 => {
                let user = User::from(
                    res.first()
                        .expect("err1")
                        .result
                        .as_ref()
                        .expect("err2")
                        .clone(),
                );
                return Ok(user);
            }
            _ => return Err("Multiple users found".to_string()),
        };
    }
    // async fn get_all(args: Arg) -> Vec<User> {}
    // async fn create(&self) -> User {}
    // async fn update(&self) -> User {}
    // async fn delete(&self) -> bool {}
}

impl From<surrealdb::sql::Value> for User {
    fn from(value: surrealdb::sql::Value) -> Self {
        match value {
            Value::Array(d) => {
                for val in d.as_slice().iter() {
                    return User::from(val.clone());
                }
                panic!("Empty array from db")
            }
            Value::Object(d) => {
                let email =
                    EmailAddress::from_str(d.get("email").unwrap().clone().as_string().as_str())
                        .expect("bad email");
                let password = Password::new(d.get("password").unwrap().clone().as_string());
                return User {
                    id: d.get("id").unwrap().clone().as_string(),
                    email,
                    name: d.get("name").unwrap().clone().as_string(),
                    password,
                    is_admin: false,
                    is_active: false,
                    created: Utc::now(),
                    modified: Utc::now(),
                };
            }
            _ => {
                panic!("Unable to convert db values to User")
            }
        }
    }
}
