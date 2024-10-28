use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let pool = configure_database(&configuration.database)
        .await
        .expect("Failed to connect to Postgres.");
    let server = startup::run(listener, pool.clone()).expect("Failed to spawn app");
    let _ = actix_web::rt::spawn(server);
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool: pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> Result<PgPool, sqlx::Error> {
    let id: Uuid = Uuid::new_v4();
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, id).as_str())
        .await
        .expect("Failed to create database.");
    // Migrate database
    let connection_pool = PgPool::connect(&format!(
        "{}/{}",
        &config.connection_string_without_db(),
        id
    ))
    .await
    .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    Ok(connection_pool)
}
