use axum::response::Html;

mod homepage;
pub use homepage::home_renderer;

mod about;
pub use about::about_renderer;

mod not_found;
pub use not_found::not_found_renderer;

use crate::{elements::Element, utils::Utils};

pub trait Page {
    fn title(&self)->String;
    fn components(&self)->&[Box<dyn Element>];

    fn data(&self)->Html<String>{
        let mut stlyes = String::new();
        let mut layouts = String::new();
        let mut scripts = String::new();

        for component in self.components(){
            let init  = component.html();
            layouts.push_str(init.layout.as_str());
            scripts.push_str(init.script.as_str());
            stlyes.push_str(init.style.as_str());
        }

        let global_style = Utils::load_style("style.css");

        let init = format!(
            r#"<html>
                <head>
                    <title>{}</title>
                    <script src='https://unpkg.com/alpinejs' defer></script>
                    <style>{}{}</style>
                </head>
                <body>{}</body>
            </html>"#, self.title(), global_style, stlyes, layouts, 
        );

        return Html(init);
    }
}