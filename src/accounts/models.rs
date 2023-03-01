use std::str::FromStr;
use chrono::{DateTime, Utc};
use email_address::EmailAddress;
use passwords::{analyzer, hasher};
use serde::__private::de;
use surrealdb::sql::{Value, Part};

pub struct Password {
    hashed_password: String,
}

impl Password {
    pub fn new(password: String) -> Result<Password, Vec<String>> {
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
        unsafe {
            hasher::identify_bcrypt_format(&password.as_str(), &self.hashed_password)
        }
    }

    fn hash_password(password: &String) -> String {
        let salt = hasher::gen_salt();
        hasher::bcrypt_format(10, &salt, &password).expect("unable to hash password")
    }

    pub fn password(self) -> String {
        self.hashed_password
    }
}

#[derive(serde::Serialize)]
pub struct User {
    pub email: EmailAddress,
    #[serde(skip_serializing)]
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
        let hashed_password = Password::new(password)?;

        let user = User {
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

    pub fn get(email: String) -> Result<User, Vec<String>> {
        let valid_email = EmailAddress::from_str(email.as_str()).expect("bad email");
        let password = Password::new("Test1".to_string())?;
        let user = Self {
            email: valid_email,
            password: password,
            name: "Test".to_string(),
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
        ].into()
    }
}

impl From<surrealdb::sql::Value> for User {
    fn from(value: surrealdb::sql::Value) -> Self {

        match value {
            Value::Array(d) => {
                for val in d.as_slice().iter() {
                    return User::from(val.clone())
                }
                panic!("Empty array from db")
            },
            Value::Object(d) => {
                let email = EmailAddress::from_str(d.get("email").unwrap().clone().as_string().as_str()).expect("bad email");
                let password = Password::new(d.get("password").unwrap().clone().as_string()).unwrap();
                return User {
                    email,
                    name: d.get("name").unwrap().clone().as_string(),
                    password,
                    is_admin: false,
                    is_active: false,
                    created: Utc::now(),
                    modified: Utc::now(),
                }
            },
            _ => { panic!("Unable to convert db values to User") }
        }
    }

}
