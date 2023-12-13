use crate::user::User;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashSet;
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlPermissionTokens {
    pub token: Cow<'static, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlUser {
    pub id: Option<Thing>,
    pub email: Cow<'static, str>,
    pub password: Cow<'static, str>,
}

impl SqlUser {
    pub fn into_user(self, sql_user_perms: Option<Vec<SqlPermissionTokens>>) -> User {
        User {
            id: self.id.unwrap().id.to_raw().parse::<i64>().unwrap(),
            email: self.email.to_string(),
            password: self.password.to_string(),
            permissions: if let Some(user_perms) = sql_user_perms {
                user_perms
                    .into_iter()
                    .map(|x| x.token.to_string())
                    .collect::<HashSet<String>>()
            } else {
                HashSet::<String>::new()
            },
        }
    }
}
