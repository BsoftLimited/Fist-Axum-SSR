use axum::response::Html;

use crate::elements::{Element, Header, NotFoundBody};
use super::Page;


struct NotFoundPage{
    components: Vec<Box<dyn Element>>
}

impl NotFoundPage {
    fn new()->Self{
        NotFoundPage{ components: vec![
            Box::new(Header::new("")),
            Box::new(NotFoundBody),
        ] }
    }
}

impl Page for NotFoundPage {
    fn title(&self)->String {
        "Axum Test | Page Not Found".to_owned()
    }

    fn components(&self)->&[Box<dyn Element>] {
        return  &self.components;
    }
}

pub async fn not_found_renderer() -> Html<String> {
    let init = NotFoundPage::new();
    
    return init.data();
}