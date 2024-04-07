use crate::user::User;
use leptos::*;

#[server(Foo, "/api")]
pub async fn foo() -> Result<String, ServerFnError<String>> {
    Ok(String::from("Bar!"))
}

#[server(GetUser, "/api")]
pub async fn get_user() -> Result<Option<User>, ServerFnError<String>> {
    let auth = crate::server::auth()?;

    Ok(auth.current_user)
}

#[server(Login, "/api")]
pub async fn login(
    email: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError<String>> {
    let pool = crate::server::pool()?;
    let auth = crate::server::auth()?;

    let user: User = User::get_from_username(email, &pool)
        .await
        .ok_or_else(|| ServerFnError::WrappedServerError("User does not exist.".to_string()))?;

    match bcrypt::verify(password, &user.password)
        .map_err(|e| ServerFnError::WrappedServerError(e.to_string()))?
    {
        true => {
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());
            leptos_axum::redirect("/");
            Ok(())
        }
        false => Err(ServerFnError::WrappedServerError(
            "Password does not match.".to_string(),
        )),
    }
}

#[server(Signup, "/api")]
pub async fn signup(
    email: String,
    password: String,
    password_confirmation: String,
    remember: Option<String>,
) -> Result<(), ServerFnError<String>> {
    use crate::server::SqlUser;
    use bcrypt::{hash, DEFAULT_COST};
    let pool = crate::server::pool()?;
    let auth = crate::server::auth()?;

    if password != password_confirmation {
        return Err(ServerFnError::WrappedServerError(
            "Passwords did not match.".to_string(),
        ));
    }

    let password_hashed = hash(password, DEFAULT_COST).unwrap();

    let mut user_res = pool
        .query(
            "LET $count = (SELECT count() FROM users GROUP BY count)[0].count + 1;
            CREATE users SET id = $count, email = $email, password = $password;",
        )
        .bind(("email", email.clone()))
        .bind(("password", password_hashed))
        .await
        .map_err(|e| ServerFnError::WrappedServerError(e.to_string()))?;

    let _sqluser: Option<SqlUser> = user_res
        .take(0)
        .map_err(|e| ServerFnError::WrappedServerError(e.to_string()))?;

    let user = User::get_from_username(email, &pool).await.ok_or_else(|| {
        ServerFnError::WrappedServerError("Signup failed: User does not exist.".to_string())
    })?;

    auth.login_user(user.id);
    auth.remember_user(remember.is_some());

    leptos_axum::redirect("/");

    Ok(())
}

#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError<String>> {
    let auth = crate::server::auth()?;

    auth.logout_user();
    leptos_axum::redirect("/");

    Ok(())
}
