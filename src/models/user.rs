use axum::{response::{IntoResponse, Response}, extract::{Query, State}, http::StatusCode};
use pgdb_lib_rs::Database;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Debug, Clone)]
pub struct CreateUserRequest {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize, Debug, Clone)]
struct CreateUserResponse{
    id: u64,
    pub name: String,
    pub surname: String,
    pub email: String
}

impl IntoResponse for CreateUserResponse {
    fn into_response(self) -> Response {
        (StatusCode::from_u16(201).unwrap(), serde_json::to_string(&self).unwrap()).into_response()
    }
}

pub struct UserDetails{
    pub name: String,
    pub surname: String,
    pub email: String,
}

struct User;

impl User {
    pub async fn write(request: &CreateUserRequest, db: &Database) -> sqlx::Result<u64> {
        let result = sqlx::query("insert into users(name, surname, email, password) values ($1 , $2, $3, $4)")
            .bind(request.name.clone())
            .bind(request.surname.clone())
            .bind(request.email.clone())
            .bind(request.password.clone())
            .execute(db.get_pool()).await?;
        Ok(result.rows_affected())
    }
}

pub async fn user_create( request: Option<Query<CreateUserRequest>>) -> impl IntoResponse {

    //let Query(paginat) = request.unwrap_or_default();

    CreateUserResponse{ id: 1, name: String::from("Nobel"), surname: "Okelekele".to_owned(), email: "okelekelenobel@gmail.com".to_owned() }
}

pub async fn user_details( request: Option<Query<CreateUserRequest>>) -> impl IntoResponse {

    //let Query(paginat) = request.unwrap_or_default();

    CreateUserResponse{ id: 1, name: String::from("Nobel"), surname: "Okelekele".to_owned(), email: "okelekelenobel@gmail.com".to_owned() }
}
