/*
 * /src/main.rs
*/

// Module definition(s)
use sqlx::sqlite::SqlitePoolOptions;
use xar2::controllers::routes;
use xar2::models::Email;

/********* CONSTANTS **********************************************************/
const HOST: [u8; 4] = [127, 0, 0, 1];
const PORT: u16 = 5000;

/********* ENTRY POINT ********************************************************/
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create SQLite pool from app.db
    let pool = SqlitePoolOptions::new().connect("./app.db").await?;

    // Ensure emails table exists 
    match sqlx::query_as::<_, Email>("SELECT * FROM emails")
        .fetch_all(&pool)
        .await
    {
        Ok(emails) => println!("[LOG] EMAILS: {:?}", emails),
        Err(e) => {
            eprintln!("[STATUS] FATAL ERROR: {}", e);
            return Err(e);
        }
    }

    // Serve routes on HOST:PORT
    warp::serve(routes(pool)).run((HOST, PORT)).await;

    println!("[STATUS] SERVER LISTENING ON {}", &PORT);

    Ok(())
}
