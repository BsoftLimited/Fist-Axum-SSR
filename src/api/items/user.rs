use axum::{http::{StatusCode, HeaderMap, Response, HeaderValue, self}, Json, Extension};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Pool, FromRow};
use tower_cookies::{cookie::{CookieBuilder, time::{OffsetDateTime, UtcOffset, macros::offset, Duration}}, Cookies};

use crate::api::config::{Database, ResultHandler, Session, ErrorResponse};
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub struct CreateUserRequest {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize, Debug, Clone, FromRow)]
pub struct UserDetails{
    pub name: String,
    pub surname: String,
    pub email: String,
}


#[derive(Deserialize, Debug, Clone)]
pub struct LoginRequest{
    email: String, password: String
}

#[derive(Serialize, Debug, Clone, FromRow)]
pub struct User{ id: String, name: String, surname: String, email: String, }

impl User {
    pub async fn write(pool: &Pool<Postgres>, request: &CreateUserRequest) -> ResultHandler<String> {
        let userID = Uuid::new_v4().to_string();
        let result = sqlx::query("INSERT into Users(id, name, surname, email, password) values ($1 , $2, $3, $4, $5)")
            .bind(&userID)
            .bind(request.name.clone())
            .bind(request.surname.clone())
            .bind(request.email.clone())
            .bind(request.password.clone())
            .execute(pool).await;
        if let Err(error) = result{
            return ResultHandler::Error { code: 500, message: "unable  to register user".to_owned(), error: error.to_string() };
        }
        
        return Session::create(pool, &userID).await;
    }

    pub async fn get(pool: &Pool<Postgres>, id: &str)-> ResultHandler<UserDetails>{
        let session_result = Session::get(pool, id).await;
        if let ResultHandler::Error { code, message, error } = session_result{
            return ResultHandler::Error { code, message, error };
        }

        let userID = session_result.unwrap(); println!("userID: {}", userID);
        let result = sqlx::query_as::<_, UserDetails>("SELECT name, surname, email FROM Users WHERE id = $1").bind(userID).fetch_one(pool).await;
        if let Err(error) = result{
            return ResultHandler::Error { code: 400, message: "unable to fetch user details".to_owned(), error: format!("{}", error) };
        }
        return ResultHandler::Pass(result.unwrap());
    }

    pub async fn all(pool: &Pool<Postgres>)-> ResultHandler<Vec<UserDetails>>{
        let result = sqlx::query_as::<_, UserDetails>("SELECT name, surname, email FROM Users").fetch_all(pool).await;
        if let Err(error) = result{
            return ResultHandler::Error { code: 400, message: "unable to fetch user details".to_owned(), error: format!("{}", error) };
        }
        return ResultHandler::Pass(result.unwrap());
    }

    pub async fn login(pool: &Pool<Postgres>, request: &LoginRequest)->ResultHandler<(String, UserDetails)>{
      
        let result = sqlx::query_as::<_, User>("SELECT id, name, surname, email FROM Users WHERE email = $1 AND password = $2")
            .bind(&request.email).bind(&request.password).fetch_one(pool).await;
        if let Err(error) = result{
            return ResultHandler::Error { code: 400, message: "invalid email or passowrd".to_owned(), error: format!("{}", error) };
        }

        let details = result.unwrap();
        let session_result = Session::create(pool, &details.id).await;
        if let ResultHandler::Error { code, message, error } = session_result{
            return ResultHandler::Error { code, message, error };
        }
        return ResultHandler::Pass((session_result.unwrap().clone(), UserDetails{ name: details.name, surname: details.surname, email: details.email }));
    }

    pub async fn logout(pool: &Pool<Postgres>, id: &str)->ResultHandler<bool>{
        let session_result = Session::get(pool, id).await;
        if let ResultHandler::Error { code, message, error } = session_result{
            return ResultHandler::Error { code, message, error };
        }
        return Session::delete(pool, session_result.unwrap()).await;
    }


}

pub async fn create_user(Json(payload): Json<CreateUserRequest>) -> Response<String> {
    let db_result =  Database::new().await;
    if db_result.is_error(){
        return db_result.to_error_response();
    }

    let pool = db_result.unwrap();
    let result = User::write(pool, &payload).await;
    if result.is_error(){
        return result.to_error_response();
    }

    let expiration_time = OffsetDateTime::now_utc().checked_add(Duration::days(7)).unwrap();
    let cookie = CookieBuilder::new("axum", result.unwrap()).http_only(true).expires(expiration_time).finish();
    let header_value = HeaderValue::from_str(&cookie.to_string()).unwrap();

    // Create a `Set-Cookie` header with the cookie value
    let set_cookie_header = http::header::SET_COOKIE.to_string().parse::<http::header::HeaderName>().unwrap();


    let details = UserDetails{ name: payload.name, surname: payload.surname, email: payload.email };

    let mut response = Response::new(serde_json::to_string(&details).unwrap());
    response.headers_mut().insert(set_cookie_header, header_value);
    response.headers_mut().insert(http::header::CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
    *response.status_mut() = StatusCode::CREATED;

    return response;
}

pub async fn init_user(cookies: Cookies) -> Response<String>{
    let cookie = cookies.get("axum");
     //= headers.get("axum").and_then(|cookie_header| cookie_header.to_str().ok()).and_then(|cookie_str| cookie_str.split(';').next()).unwrap_or("");

    if cookie.is_none(){
        let error_response = ErrorResponse{ message: String::from("expired or invalid session, try login in") };
        
        let mut response = Response::new(serde_json::to_string(&error_response).unwrap());
        *response.status_mut() = StatusCode::BAD_REQUEST;
        response.headers_mut().insert(http::header::CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());

        return response;
    }

    let id = match cookie {
            Some(val) => val.value().to_owned(),
            None => panic!("called `Option::unwrap()` on a `None` value"),
    };

    let db_result = Database::new().await;
    if db_result.is_error(){
        return db_result.to_error_response();
    }

    let pool = db_result.unwrap();
    
    println!("cookie: {}", id);
    let details_result = User::get(pool, id.as_str()).await;
    if details_result.is_error(){
        return details_result.to_error_response();
    }

    let mut response = Response::new(serde_json::to_string(details_result.unwrap()).unwrap());
    *response.status_mut() = StatusCode::OK;
    response.headers_mut().insert(http::header::CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());

    return response;
    
}

pub async fn all_users() -> Response<String> {
    let db_result = Database::new().await;
    if db_result.is_error(){
        return db_result.to_error_response();
    }

    let pool = db_result.unwrap();
    
    let users_result = User::all(pool).await;
    if users_result.is_error(){
        return users_result.to_error_response();
    }

    let mut response = Response::new(serde_json::to_string(users_result.unwrap()).unwrap());
    *response.status_mut() = StatusCode::OK;
    response.headers_mut().insert(http::header::CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());

    return response;
}

pub async fn login_user(Json(payload): Json<LoginRequest>)->Response<String>{
    let db_result = Database::new().await;
    if db_result.is_error(){
        return db_result.to_error_response();
    }

    let pool = db_result.unwrap();
    
    let users_result = User::login(pool, &payload).await;
    if users_result.is_error(){
        return users_result.to_error_response();
    }

    let (id, details) = users_result.unwrap();

    let expiration_time = OffsetDateTime::now_utc().checked_add(Duration::days(7)).unwrap();
    let cookie = CookieBuilder::new("axum", id).http_only(true).expires(expiration_time).finish();
    let header_value = HeaderValue::from_str(&cookie.to_string()).unwrap();

    // Create a `Set-Cookie` header with the cookie value
    let set_cookie_header = http::header::SET_COOKIE.to_string().parse::<http::header::HeaderName>().unwrap();

    let mut response = Response::new(serde_json::to_string(details).unwrap());
    response.headers_mut().insert(set_cookie_header, header_value);
    response.headers_mut().insert(http::header::CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
    *response.status_mut() = StatusCode::CREATED;

    return response;
}
