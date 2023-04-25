mod database;
use axum::http::{StatusCode, Response, HeaderValue, self};
pub use database::Database;

mod session;
use serde::Serialize;
pub use session::Session;

#[derive(Serialize, Debug, Clone)]
pub struct ErrorResponse{ pub message: String }

#[derive(Debug)]
pub enum ResultHandler<T>{
    Pass(T), Error{code: u16, message: String, error: String }
}

impl <T>ResultHandler<T>{
    pub fn is_error(&self)->bool{
        match self {
            Self::Pass(_) => return false,
            Self::Error{ code: _, message:_, error:_ } => return true
        }
    }

    pub fn unwrap(&self)->&T{
        match self {
            ResultHandler::Pass(value) => return value,
            ResultHandler::Error{ code, message:_, error } =>{ panic!("Result handler is an  error variant: code: {}, error: {}", code, error) }
        }
    }

    pub fn to_error_response(&self)-> Response<String>{
        match self {
            ResultHandler::Error { code, message, error } =>{
                eprintln!("{}", error);
                let response = ErrorResponse{ message: message.clone() };

                let body = serde_json::to_string(&response).unwrap();

                let mut response = Response::new(body);
                response.headers_mut().insert(http::header::CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
                *response.status_mut() = StatusCode::from_u16(code.clone()).unwrap();
                
                return response;
            },
            ResultHandler::Pass(_) => { panic!("Result handler is not of variant error"); }
        }
    }
}