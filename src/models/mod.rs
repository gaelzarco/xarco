#[derive(Debug, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
pub struct Email {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub body: String,
}
