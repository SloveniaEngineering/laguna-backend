use laguna_backend_model::user::{Role, User};
use sqlx::{postgres::PgPoolOptions, PgPool};

const TEST_DATABASE_URL: &str = "postgres://postgres:postgres@127.0.0.1:5432/laguna_test_db";

pub(crate) async fn setup() -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(TEST_DATABASE_URL)
        .await
        .expect("Unable to connect to test database");

    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("Couldn't run migrations");

    pool
}

pub(crate) async fn teardown(pool: PgPool) {
    pool.close().await
}

#[actix_web::test]
async fn crud_user() {
    let conn = setup().await;
    sqlx::query(
        r#"
    INSERT INTO "User" (username, email, password, avatar_url, role) 
    VALUES ('test', 'test@laguna.io', digest('test123', 'sha-256'), NULL, $1)
    "#,
    )
    .bind(Role::Admin)
    .execute(&conn)
    .await
    .expect("INSERT failed");
    let user = sqlx::query_as::<_, User>("SELECT * FROM \"User\"")
        .fetch_one(&conn)
        .await
        .expect("Could not get user");
    assert_eq!(
        user,
        User {
            id: user.id,
            username: String::from("test"),
            password: user.password.clone(),
            email: String::from("test@laguna.io"),
            first_login: user.first_login,
            last_login: user.last_login,
            avatar_url: None,
            role: Role::Admin,
        }
    );
    sqlx::query("DELETE FROM \"User\" WHERE id = $1")
        .bind(user.id)
        .execute(&conn)
        .await
        .expect("Couldn't delete user");
    teardown(conn).await;
}
