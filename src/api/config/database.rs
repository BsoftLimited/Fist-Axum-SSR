use axum::http::StatusCode;
use sqlx::postgres::{PgPoolOptions};
use sqlx::{ Pool, Postgres};

use super::ResultHandler;


struct DatabaseConfig{
    host: String,
    user: String,
    password: String,
    db: String,
}

impl DatabaseConfig{
    pub fn new(host: &str, user: &str, password: &str, db: &str)->Self{
        DatabaseConfig { host: host.to_owned(), user: user.to_owned(), password: password.to_ascii_lowercase(), db: db.to_owned() }
    }

    fn connection(&self)->String{
        return format!("postgres://{}:{}@{}/{}", self.user, self.password, self.host, self.db);
    }
}

pub struct Database;

impl Database{
    pub async fn new()->ResultHandler<Pool<Postgres>>{
        let config = DatabaseConfig::new("localhost", "bsoft", "rustup", "axum");
        match PgPoolOptions::new().max_connections(5).connect(&config.connection()).await{
            Ok(pool) =>{
                if let ResultHandler::Error { code, message, error } = Database::check_and_create_tables_if_not_exists(&pool).await{
                    return ResultHandler::Error { code, message, error };
                }
                return ResultHandler::Pass(pool);
            },
            Err(error) =>{
                return ResultHandler::Error { code: 500, message: "unable to connect to the database".to_owned(), error: format!("{}", error) };
            }
        }
    }

    async fn create_table(pool: &Pool<Postgres>, query: &str) ->ResultHandler<bool> {
        match sqlx::query(query).execute(pool).await {
            Ok(_) => { return ResultHandler::Pass(true); },
            Err(error) => {
                return ResultHandler::Error{code: StatusCode::BAD_REQUEST.as_u16(),  message: "internal server error".to_owned(), error:format!("{}", error)};
            }
        }
    }

    async fn check_and_create_tables_if_not_exists(pool: &Pool<Postgres>) -> ResultHandler<bool>{
        let user_table_result = Database::create_table(pool,
                "CREATE TABLE IF NOT EXISTS Users(id CHAR(50) NOT NULL,
                    name VARCHAR(30) NOT NULL,
                    surname VARCHAR(30) NOT NULL,
                    email VARCHAR(50) NOT NULL,
                    password CHAR(50) NOT NULL, 
                    registered TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    PRIMARY KEY (id));").await;

        if user_table_result.is_error(){
            return user_table_result;
        }

        let session_table_result = Database::create_table(pool,
            "CREATE TABLE IF NOT EXISTS Sessions (id CHAR(50) NOT NULL,
                userID CHAR(50) NOT NULL, 
                PRIMARY KEY (id),
                FOREIGN KEY (userID) REFERENCES Users(id))").await;
        return session_table_result;
    }
}