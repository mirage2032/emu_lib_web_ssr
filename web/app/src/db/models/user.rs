#[cfg(not(target_arch = "wasm32"))]
use crate::db::models::schema::users::dsl;
use crate::db::models::session::*;
#[cfg(not(target_arch = "wasm32"))]
use crate::db::models::user::dsl::users;
use crate::db::password;
#[cfg(not(target_arch = "wasm32"))]
use crate::db::DbPool;
#[cfg(not(target_arch = "wasm32"))]
use diesel::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(diesel_derive_enum::DbEnum))]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ExistingTypePath = "crate::db::models::schema::sql_types::Usertype"
)] // Specify the underlying SQL type
pub enum UserType {
    User,
    Admin,
}
#[cfg_attr(not(target_arch = "wasm32"), derive(Queryable))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub user_type: UserType,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserLogin {
    pub login: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailNoPasswordLogin {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserData {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub user_type: UserType,
}

impl From<User> for UserData {
    fn from(user: User) -> Self {
        UserData {
            id: user.id,
            username: user.username,
            email: user.email,
            user_type: user.user_type,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl UserLogin {
    pub fn new(login: String, password: String) -> Self {
        UserLogin { login, password }
    }

    pub fn authenticate(
        &self,
        pool: &DbPool,
        duration: time::Duration,
    ) -> Result<(User, Session), Box<dyn Error>> {
        if let Some(user) = User::get_by_login(&self.login, pool)? {
            if password::verify_password(&self.password, &user.password_hash).is_err() {
                return Err("Invalid password".into());
            }
            let new_session = NewSession::new(user.id, duration);
            let session = Session::create(new_session, pool)?;
            Ok((user, session))
        } else {
            Err("User not found".into())
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl EmailNoPasswordLogin {
    pub fn new(email: String) -> Self {
        EmailNoPasswordLogin { email }
    }

    //Marked UNSAFE, just to make sure developer uses it carefully
    pub unsafe fn authenticate(
        &self,
        pool: &DbPool,
        duration: time::Duration,
    ) -> Result<(User, Session), Box<dyn Error>> {
        if let Ok(user) = User::get_by_email(&self.email, pool) {
            let new_session = NewSession::new(user.id, duration);
            let session = Session::create(new_session, pool)?;
            Ok((user, session))
        } else {
            Err("User not found".into())
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Insertable))]
#[cfg_attr(not(target_arch = "wasm32"), diesel (table_name = super::schema::users))]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}
impl NewUser {
    pub fn new(username: String, email: String, password: String) -> Result<Self, Box<dyn Error>> {
        let password_hash =
            password::hash_password(&password).map_err(|_| "Couldn't hash password")?;
        Ok(NewUser {
            username,
            email,
            password_hash,
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl User {
    pub fn add_user(new_user: NewUser, pool: &DbPool) -> Result<User, Box<dyn Error>> {
        let mut conn = pool.get()?;
        let user = diesel::insert_into(dsl::users)
            .values(&new_user)
            .get_result(&mut conn)?;
        Ok(user)
    }
    pub fn get_by_id(p_id: i32, pool: &DbPool) -> Result<User, Box<dyn Error>> {
        let mut conn = pool.get()?;
        let user = dsl::users
            .filter(dsl::id.eq(p_id))
            .first(&mut conn)
            .map_err(|e| e)?;
        Ok(user)
    }

    pub fn get_by_username(p_username: &str, pool: &DbPool) -> Result<User, Box<dyn Error>> {
        let mut conn = pool.get()?;
        let user = dsl::users
            .filter(dsl::username.eq(p_username))
            .first(&mut conn)
            .map_err(|e| e)?;
        Ok(user)
    }

    pub fn get_by_email(p_email: &str, pool: &DbPool) -> Result<User, Box<dyn Error>> {
        let mut conn = pool.get()?;
        let user = dsl::users.filter(dsl::email.eq(p_email)).first(&mut conn)?;
        Ok(user)
    }

    pub fn get_by_login(login: &str, pool: &DbPool) -> Result<Option<User>, Box<dyn Error>> {
        let mut conn = pool.get()?;
        match dsl::users
            .filter(dsl::username.eq(login).or(dsl::email.eq(login)))
            .first(&mut conn)
        {
            Ok(user) => Ok(Some(user)),
            Err(_) => Ok(None),
        }
    }

    pub fn delete(&self, pool: &DbPool) -> Result<(), Box<dyn Error>> {
        let mut conn = pool.get()?;
        diesel::delete(dsl::users.find(self.id)).execute(&mut conn)?;
        Ok(())
    }
}
