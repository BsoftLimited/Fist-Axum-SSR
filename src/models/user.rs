use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
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

#[derive(Serialize, Debug, Clone)]
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

pub async fn create_user(Json(payload): Json<CreateUserRequest>) -> (StatusCode, Json<UserDetails>) {
    let details = UserDetails{ name: payload.name, surname: payload.surname, email: payload.email };
    return (StatusCode::CREATED, Json(details));
}

pub async fn init_user() -> (StatusCode, Json<UserDetails>) {
    let details = UserDetails{ name: String::from("Nobel"), surname: "Okelekele".to_owned(), email: "okelekelenobel@gmail.com".to_owned() };
    return (StatusCode::OK, Json(details));
}

pub async fn all_user() -> (StatusCode, Json<Vec<UserDetails>>) {
    let users = vec![
        UserDetails{ name: String::from("Nobel"), surname: "Okelekele".to_owned(), email: "okelekelenobel@gmail.com".to_owned() },
        UserDetails{ name: String::from("Benita"), surname: "Okelekele".to_owned(), email: "benitaokelekele@gmail.com".to_owned() },
        UserDetails{ name: String::from("Victoria"), surname: "Okelekele".to_owned(), email: "victoriaokelekele@gmail.com".to_owned() },
        UserDetails{ name: String::from("Kiensue"), surname: "Okelekele".to_owned(), email: "okelekelekiensue@yahoo.com".to_owned() },
        UserDetails{ name: String::from("Ifeafa"), surname: "Okelekele".to_owned(), email: "ifeafaokelekele@gmail.com".to_owned() }
    ];
    return (StatusCode::OK, Json(users));
}
