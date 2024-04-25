use std::env;
use axum::Router;
use axum::routing::get;
use sqlx::postgres::PgPoolOptions;

mod domain;
mod tests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_path(".env").ok();
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_owned());
    let addr = format!("0.0.0.0:{}", port);

    // let db_addr =
    //     env::var("DATABASE_URL")
    //         .expect("missing DATABASE_URL env");

    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect(&db_addr)
    //     .await?;

    let app = Router::new()
        .route("/helloworld", get(|| async { "Hello, World!" }));
        // .with_state(pool);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

// async fn helloworld() -> &'static str {
//     "Hello world"
// }
