use std::str::FromStr;
use chrono::{DateTime, Utc};
use email_address::EmailAddress;
use passwords::{analyzer, hasher};

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

    pub unsafe fn check_password(self, password: String) -> bool {
        hasher::identify_bcrypt_format(password.as_str(), self.hashed_password)
    }

    fn hash_password(password: &String) -> String {
        let salt = hasher::gen_salt();
        hasher::bcrypt_format(10, &salt, &password).unwrap()
    }
}

pub struct User {
    pub email: EmailAddress,
    pub password: Password,
    pub name: String,
    pub is_admin: bool,
    pub is_active: bool,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}

impl User {
    pub fn new(email: String, password: String, name: String) -> Result<User, Vec<String>> {
        let valid_email = EmailAddress::from_str(email.as_str()).unwrap();
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
}
