use crate::server_fns::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Login(action: Action<Login, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <ActionForm action=action>
            <h1>"Log In"</h1>
            <label>
                "Email:"
                <input
                    type="text"
                    placeholder="Email"
                    maxlength="32"
                    name="email"
                    class="auth-input"
                />
            </label>
            <br/>
            <label>
                "Password:"
                <input type="password" placeholder="Password" name="password" class="auth-input"/>
            </label>
            <br/>
            <label>
                <input type="checkbox" name="remember" class="auth-input"/>
                "Remember me?"
            </label>
            <br/>
            <button type="submit" class="button">
                "Log In"
            </button>
        </ActionForm>
    }
}
