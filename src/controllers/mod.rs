/*
 * /src/controllers/mod.rs
*/

use warp::{
    Filter,
    reply::json
};

use crate::{
    models::Email,
    helpers::json_body
};

/******** ROUTES *************************************************************/
/// Route Handler
///
/// Requires SqlitePool for post_email service
///
/// Defines routes and returns a warp::Filter object to use directly as a
/// parameter for warp::serve().
/// 
/// For Example:
/// ```rust
/// warp::serve(routes(pool)).run((HOST, PORT)).await;
/// ```
pub fn routes(pool: sqlx::SqlitePool) -> 
impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Index
    let idx = warp::path::end()
        .and(warp::fs::file("./src/views/idx/index.html"));

    // Static 
    let static_files = warp::path("static").and(warp::fs::dir("./static"));

    // Emails
    let email = warp::post()
        .and(warp::path("contact"))
        .and(warp::path("email"))
        .and(json_body())
        .and(warp::any().map(move || pool.clone()))
        .and_then(post_email);

    // Routes
    idx.or(email).or(static_files)
}

/******** ROUTE HANDLERS ******************************************************/
/// Email Post Service
///
/// Takes in a user-submitted email and a database pool and submits the email
/// to the database.
///
/// Prints whether the write is successful or not.
pub async fn post_email(
    msg: Email,
    pool: sqlx::SqlitePool,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("[LOG]: New Email Received: {:?}", msg);

    // Insert data into emails table
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

    // Verify email write
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
