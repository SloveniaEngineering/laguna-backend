use sqlx::types::{time::OffsetDateTime, Uuid};

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
    /// UTC DateTime aka TIMESTAMP WITH TIME ZONE
    pub first_login: OffsetDateTime,
    /// UTC DateTime aka TIMESTAMP WITH TIME ZONE
    pub last_login: OffsetDateTime,
    pub avatar_url: Option<String>,
    pub role: Role,
}
