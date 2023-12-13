use crate::routes::*;
use crate::server_fns::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    let login = create_server_action::<Login>();
    let logout = create_server_action::<Logout>();
    let signup = create_server_action::<Signup>();

    let auth_change = move || {
        login.version().with(|first| {
            signup.version().with(|middle| {
                logout
                    .version()
                    .with(|last| format!("{} {} {}", first, middle, last))
            })
        })
    };

    let user = create_resource(move || auth_change(), move |_| get_user());

    provide_meta_context();

    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/test.css"/>
        <Title text="Welcome to Leptos Surrealdb Axum Session Auth"/>
        <Router>
            <header>
                <A href="/">
                    <h1>"Auth"</h1>
                </A>
                <Transition fallback=move || {
                    view! { <span>"Loading..."</span> }
                }>
                    {move || {
                        user.get()
                            .map(|user| match user {
                                Err(e) => {
                                    view! {
                                        <A href="/signup">"Signup"</A>
                                        " | "
                                        <A href="/login">"Login"</A>
                                        " | "
                                        <span>{format!("Login error: {}", e)}</span>
                                    }
                                }
                                Ok(None) => {
                                    view! {
                                        <A href="/signup">"Signup"</A>
                                        " | "
                                        <A href="/login">"Login"</A>
                                        " | "
                                        <span>"Logged out."</span>
                                    }
                                }
                                Ok(Some(user)) => {
                                    view! {
                                        <A href="/settings">"Settings"</A>
                                        " | "
                                        <span>
                                            {format!("Logged in as: {} ({})", user.email, user.id)}
                                        </span>
                                    }
                                }
                            })
                    }}

                </Transition>
            </header>
            <hr/>
            <main>
                <Routes>
                    // Route
                    <Route path="" view=HomePage/>
                    <Route path="signup" view=move || view! { <Signup action=signup/> }/>
                    <Route path="login" view=move || view! { <Login action=login/> }/>
                    <Route
                        path="settings"
                        view=move || {
                            view! {
                                <h1>"Settings"</h1>
                                <Logout action=logout/>
                            }
                        }
                    />

                </Routes>
            </main>
        </Router>
    }
}
