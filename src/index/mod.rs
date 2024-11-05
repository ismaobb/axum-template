use axum::{routing::post, Router};

mod login;
mod logout;

pub fn controller() -> Router {
    Router::new().nest_service(
        "/",
        Router::new()
            .route("/login", post(login::login))
            .route("/logout", post(logout::logout)),
    )
}
