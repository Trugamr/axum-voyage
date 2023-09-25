use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/hello", get(hello_handler))
        .route("/blog/:slug", get(blog_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root_handler() -> &'static str {
    "welcome 🙏"
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn hello_handler(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("there");
    Html(format!(
        r#"
        <html>
            <head>
                <title>voyage</title>
            </head>
            <body style="color: #eee; background-color: #0f0f0f;">
                <h1>hello {name} 🙂</h1>
            </body>
        </html>
        "#,
    ))
}

async fn blog_handler(Path(slug): Path<String>) -> impl IntoResponse {
    Html(format!(
        r#"
        <html>
            <head>
                <title>voyage</title>
            </head>
            <body style="color: #eee; background-color: #0f0f0f;">
                <h1>blog {slug} 🙂</h1>
            </body>
        </html>
        "#,
        slug = slug
    ))
}
