use super::models::*;
use crate::user::User;
use async_trait::async_trait;
use axum_session_auth::{Authentication, HasPermission, SessionSurrealPool};
use leptos::*;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub type SurrealPool = Surreal<Client>;
pub type AuthSession =
    axum_session_auth::AuthSession<User, i64, SessionSurrealPool<Client>, SurrealPool>;

pub fn pool() -> Result<SurrealPool, ServerFnError<String>> {
    use_context::<SurrealPool>()
        .ok_or_else(|| ServerFnError::WrappedServerError("Pool missing.".to_string()))
}

pub fn auth() -> Result<AuthSession, ServerFnError<String>> {
    use_context::<AuthSession>()
        .ok_or_else(|| ServerFnError::WrappedServerError("Auth session missing.".to_string()))
}

impl User {
    pub async fn get(id: i64, pool: &SurrealPool) -> Option<Self> {
        let mut user_res = pool
            .query("SELECT * FROM users WHERE meta::id(id) = $id")
            .bind(("id", id))
            .await
            .ok()?;

        let sqluser: Option<SqlUser> = user_res.take(0).ok()?;

        //lets just get all the tokens the user can use, we will only use the full permissions if modifing them.
        let mut perm_res = pool
            .query("SELECT token FROM user_permissions WHERE user_id = $id")
            .bind(("id", id))
            .await
            .ok()?;

        let sql_user_perms: Vec<SqlPermissionTokens> = perm_res.take(0).ok()?;

        Some(sqluser?.into_user(Some(sql_user_perms)))
    }

    pub async fn get_from_username(email: String, pool: &SurrealPool) -> Option<Self> {
        let mut user_res = pool
            .query("SELECT * FROM users WHERE email = $email")
            .bind(("email", email))
            .await
            .ok()?;

        let sqluser: Option<SqlUser> = user_res.take(0).ok()?;

        let mut perm_res = pool
            .query("SELECT token FROM user_permissions WHERE user_id = $id")
            .bind((
                "id",
                sqluser
                    .clone()
                    .expect("expected sqluser")
                    .id
                    .expect("expected sqluser thing")
                    .id,
            ))
            .await
            .ok()?;

        let sql_user_perms: Vec<SqlPermissionTokens> = perm_res.take(0).ok()?;

        Some(sqluser?.into_user(Some(sql_user_perms)))
    }
}

#[async_trait]
impl Authentication<User, i64, SurrealPool> for User {
    async fn load_user(userid: i64, pool: Option<&SurrealPool>) -> Result<User, anyhow::Error> {
        let pool = pool.unwrap();

        User::get(userid, pool)
            .await
            .ok_or_else(|| anyhow::anyhow!("Cannot get user"))
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn is_active(&self) -> bool {
        true
    }

    fn is_anonymous(&self) -> bool {
        false
    }
}

#[async_trait]
impl HasPermission<SurrealPool> for User {
    async fn has(&self, perm: &str, _pool: &Option<&SurrealPool>) -> bool {
        self.permissions.contains(perm)
    }
}
