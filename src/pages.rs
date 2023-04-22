use tera::Tera;
use tera::Context;
use axum::response::Html;

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
        //tera.register_filter("do_nothing", do_nothing_filter);
        tera
    };
}


pub async fn home_page() -> Html<String> {
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