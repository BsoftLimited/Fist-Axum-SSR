use sqlx::{Postgres, Pool, FromRow};
use uuid::Uuid;

use super::ResultHandler;

#[derive(Debug, FromRow)]
pub struct Session{ id: String, userid: String }

impl Session{
    pub async fn check(pool: &Pool<Postgres>, userID: &str)->ResultHandler<bool>{
        let init = sqlx::query_as::<_, Session>("SELECT * FROM Sessions WHERE userID = $1").bind(userID).fetch_all(pool).await;
        match init {
            Ok(details) => {
                return ResultHandler::Pass(!details.is_empty());
            },
            Err(error) =>{
                return ResultHandler::Error{ code: 500, message: "internal server error".to_owned(), error: format!("{}", error) };
            }
        }      
    }

    pub async fn create(pool: &Pool<Postgres>, userID: &str)->ResultHandler<String>{
        match Session::check(pool, userID).await{
            ResultHandler::Pass(has) =>{
                if has{
                    return Session::refresh(pool, userID).await;
                }
                let id = Uuid::new_v4().to_string();
                let init = sqlx::query("INSERT INTO Sessions (id, userID) VALUES ($1, $2)").bind(&id).bind(&userID).execute(pool).await;
                match init {
                    Ok(_result) =>{ return ResultHandler::Pass(id); },
                    Err(error) => { return ResultHandler::Error{code: 500, message: "session cretaion failed".to_owned(), error:format!("{}", error) }; }
                }
            },
            ResultHandler::Error{ code, message, error} =>{ return ResultHandler::Error { code, message, error } }
        }
    }

    pub async fn refresh(pool: &Pool<Postgres>, userID: &str)->ResultHandler<String>{
        let id = Uuid::new_v4().to_string();
        let init = sqlx::query("UPDATE Sessions SET id = $1 WHERE userID = $2").bind(&id).bind(&userID).execute(pool).await;
        match init {
            Ok(_result) =>{ return ResultHandler::Pass(id); },
            Err(error) => { return ResultHandler::Error{ code: 500, message:"session update failed".to_owned(), error: format!("{}", error) }; }
        }  
    }

    pub async fn get(pool: &Pool<Postgres>, id: &str) -> ResultHandler<String>{
        let init = sqlx::query_as::<_, Session>("SELECT * FROM Sessions WHERE id = $1").bind(id).fetch_one(pool).await;
        match init{
            Ok(detail) => { return ResultHandler::Pass(detail.userid); },
            Err(error) => { return ResultHandler::Error{ code: 500, message: "session not found or expired".to_owned(), error: format!("{}", error) }; }
        }
    }    

    pub async fn delete(pool: &Pool<Postgres>, userID: &str) -> ResultHandler<bool>{
        let init = sqlx::query("DELETE from Sessions WHERE userID = $1").bind(userID).execute(pool).await;
        match init{
            Ok(_result) =>{
                return ResultHandler::Pass(true);
            },
            Err(error) =>{
                return ResultHandler::Error{ code: 500, message: "error deleting previous session".to_owned(), error: format!("{}", error) };
            }
        }
	}
}