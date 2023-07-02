use std::env;

use laguna_backend_model::user::{Role, User};
use log::debug;
use sqlx::{postgres::PgPoolOptions, PgPool};

async fn setup() -> PgPool {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL not set"))
        .await
        .expect("Unable to connect to test database");

    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("Couldn't run migrations");

    pool
}

async fn teardown(pool: PgPool) {
    pool.close().await
}

#[actix_web::test]
async fn test_insert_and_select_user() {
    let pool = setup().await;

    sqlx::query(
        r#"
    INSERT INTO "User" (username, email, password, avatar_url, role) 
    VALUES ('test', 'test@laguna.io', digest('test123', 'sha-256'), NULL, $1)
    "#,
    )
    .bind(Role::Admin)
    .execute(&pool)
    .await
    .expect("INSERT failed");

    let user = sqlx::query_as::<_, User>("SELECT * FROM \"User\"")
        .fetch_one(&pool)
        .await
        .expect("Could not get user");

    debug!("{:#?}", user);

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

    teardown(pool).await;
}
