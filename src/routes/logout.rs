use crate::server_fns::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Logout(action: Action<Logout, Result<(), ServerFnError<String>>>) -> impl IntoView {
    view! {
        <div id="loginbox">
            <ActionForm action=action>
                <button type="submit" class="button">
                    "Log Out"
                </button>
            </ActionForm>
        </div>
    }
}
