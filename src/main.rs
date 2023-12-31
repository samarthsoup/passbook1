use axum::{routing::get, Router};

mod functions;
use crate::functions::{
    deposit, depositactivity, home, login, loginactivity, delete, signup, signupactivity,
    userpage, withdraw, history, withdrawactivity
};

mod database;
mod http;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/signup", get(signup).post(signupactivity))
        .route("/login", get(login).post(loginactivity))
        .route("/userpage/:userid", get(userpage))
        .route("/history/:userid", get(history))
        .route("/deposit/:userid", get(deposit).post(depositactivity))
        .route("/withdraw/:userid", get(withdraw).post(withdrawactivity))
        .route("/delete/:userid", get(delete));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


