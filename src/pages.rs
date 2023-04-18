use axum::response::Html;

use crate::{elements::{Element, About, Home, NotFound, Users}, utils::Utils};

fn page(title: &str, element: impl Element )->Html<String>{
    let global_style = Utils::load_style("style.css");
    let component = element.html();
    let init = format!(
        r#"<!DOCTYPE html>
        <html>
            <head>
                <title>{}</title>
                <script src='https://unpkg.com/alpinejs' defer></script>
                <style>{}{}</style>
            </head>
            <body>{}
                <script>{}</script>
            </body>
        </html>"#, title, global_style, component.style.as_str(), component.layout.as_str(), component.script.as_str(), 
    );

    return Html(init);
}


pub async fn home_page() -> Html<String> {
    return page("Axum Test | Home", Home::new(None));
}

pub async fn about_page() -> Html<String> {
    return page("Axum Test | About", About);
}

pub async fn not_found_page() -> Html<String> {
    return page("Axum Test | Not Found", NotFound);
}

pub async fn users_page() -> Html<String> {
    return page("Axum Test | All Users", Users);
}