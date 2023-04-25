use tera::Tera;
use tera::Context;
use axum::response::Html;
use tower_cookies::Cookies;

use crate::api::config::Database;
use crate::api::items::User;

lazy_static::lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}


pub async fn home_page(cookies: Cookies) -> Html<String> {
    let cookie = cookies.get("axum");
    if cookie.is_some(){
        let id = cookie.unwrap().value().to_owned();
           
        let db_result = Database::new().await;
        if !db_result.is_error(){
            let pool = db_result.unwrap();
    
            let details_result = User::get(pool, id.as_str()).await;
            if !details_result.is_error(){
                
            }   
        }
    }

    let page = &TEMPLATES.render("home.html", &Context::new()).unwrap();

    return Html(page.to_owned());
}

pub async fn about_page() -> Html<String> {
    let page = &TEMPLATES.render("about.html", &Context::new()).unwrap();
    return Html(page.to_owned());
}

pub async fn not_found_page() -> Html<String> {
    let page = &TEMPLATES.render("not-found.html", &Context::new()).unwrap();
    return Html(page.to_owned());
}

pub async fn users_page() -> Html<String> {
    let page = &TEMPLATES.render("users.html", &Context::new()).unwrap();
    return Html(page.to_owned());
}