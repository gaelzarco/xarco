/*
 * /src/main.rs
*/

use sqlx::sqlite::SqlitePoolOptions;
use warp::{reply::json, Filter};

#[derive(Debug, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
struct Email {
    id: u32,
    first_name: String,
    last_name: String,
    email: String,
    body: String,
}

fn json_body() -> impl Filter<Extract = (Email,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn handle_email(
    msg: Email,
    pool: sqlx::SqlitePool,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("[LOG]: New Email Received: {:?}", msg);

    let result = sqlx::query(
        "INSERT INTO emails (first_name, last_name, email, body)
        VALUES (?, ?, ?, ?)",
    )
    .bind(&msg.first_name)
    .bind(&msg.last_name)
    .bind(&msg.email)
    .bind(&msg.body)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => {
            println!("[LOG]: Successful Email Write");
            Ok(json(&msg))
        }
        Err(e) => {
            eprintln!("[ERROR]: Failed Email Write: {:?}", e);
            Err(warp::reject::reject())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePoolOptions::new().connect("./app.db").await?;

    match sqlx::query_as::<_, Email>("SELECT * FROM emails")
        .fetch_all(&pool)
        .await
    {
        Ok(_) => println!("[STATUS]: Server Started!"),
        Err(e) => {
            eprintln!("[FATAL ERROR]: {}", e);
            return Err(e);
        }
    }

    // Home Route
    let home = warp::path::end().and(warp::fs::file("./static/index.html"));

    // Serve static files
    let static_files = warp::path("static").and(warp::fs::dir("./static"));

    let email = warp::post()
        .and(warp::path("contact"))
        .and(warp::path("email"))
        .and(json_body())
        .and(warp::any().map(move || pool.clone()))
        .and_then(handle_email);

    let routes = home.or(email).or(static_files);

    warp::serve(routes).run(([127, 0, 0, 1], 5000)).await;

    Ok(())
}
