use cfg_if::cfg_if;

mod app;
mod fallback;
mod routes;
mod server;
mod server_fns;
mod user;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::fallback::file_and_error_handler;
    use crate::server::{AppState ,SurrealPool, AuthSession};
    use crate::app::App;
    use crate::user::User;
    use axum::{
        response::{Response, IntoResponse},
        routing::get,
        extract::{Path, State, RawQuery},
        http::{Request, header::HeaderMap},
        body::Body as AxumBody,
        Router,
    };
    use axum_session::{SessionConfig, SessionLayer, SessionStore, SessionSurrealPool};
    use axum_session_auth::{AuthSessionLayer, AuthConfig};
    use leptos::{logging::log, provide_context, get_configuration};
    use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};
    use surrealdb::Surreal;
    use surrealdb::engine::remote::ws::{Ws, Client};
    use surrealdb::opt::auth::Root;

    async fn server_fn_handler(State(app_state): State<AppState>, auth_session: AuthSession, path: Path<String>, headers: HeaderMap, raw_query: RawQuery,
    request: Request<AxumBody>) -> impl IntoResponse {
        handle_server_fns_with_context(path, headers, raw_query, move || {
            provide_context(auth_session.clone());
            provide_context(app_state.pool.clone());
        }, request).await
    }

    async fn leptos_routes_handler(auth_session: AuthSession, State(app_state): State<AppState>, req: Request<AxumBody>) -> Response{
            let handler = leptos_axum::render_route_with_context(app_state.leptos_options.clone(),
            app_state.routes.clone(),
            move || {
                provide_context(auth_session.clone());
                provide_context(app_state.pool.clone());
            },
            App
        );
        handler(req).await.into_response()
    }


    #[tokio::main]
    async fn main() {
        simple_logger::init_with_level(log::Level::Warn).expect("couldn't initialize logging");

        let db = Surreal::new::<Ws>("127.0.0.1:8000").await.expect("Could not make pool.");
        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await.expect("Could not make pool.");

        db.use_ns("test_namespace").use_db("test_db").await.expect("Could not make pool.");

        let pool = db.clone();

        // Auth section
        let session_config = SessionConfig::default();
        let auth_config = AuthConfig::<i64>::default();
        let session_store =
            SessionStore::new(Some(SessionSurrealPool::new(pool.clone())), session_config)
                .await
                .unwrap();

        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;
        let addr = leptos_options.site_addr;
        let routes = generate_route_list(App);

        let app_state = AppState{
            leptos_options,
            pool: pool.clone(),
            routes: routes.clone(),
        };

        let app = Router::new()
            .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
            .leptos_routes_with_handler(routes, get(leptos_routes_handler) )
            .fallback(file_and_error_handler)
            .layer(AuthSessionLayer::<User, i64, SessionSurrealPool<Client>, SurrealPool>::new(Some(pool.clone()))
            .with_config(auth_config))
            .layer(SessionLayer::new(session_store))
            .with_state(app_state);

        log!("listening on http://{}", &addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}

    else {
        pub fn main() {
            // This example cannot be built as a trunk standalone CSR-only app.
            // Only the server may directly connect to the database.
        }
    }
}
