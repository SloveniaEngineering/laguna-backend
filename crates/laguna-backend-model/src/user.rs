use sqlx::types::{time::PrimitiveDateTime, Uuid};

#[derive(Debug, PartialEq, Eq, sqlx::Type)]
pub enum Role {
    Normie,
    Verified,
    Mod,
    Admin,
}

#[derive(Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct User {
    /// Generated using uuid_generate_v4()
    pub id: Uuid,
    pub username: String,
    pub email: String,
    /// Hashed using SHA-256
    pub password: String,
    pub first_login: PrimitiveDateTime,
    pub last_login: PrimitiveDateTime,
    pub avatar_url: Option<String>,
    pub role: Role,
}
