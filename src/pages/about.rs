use axum::response::Html;

use crate::elements::{Element, Header, AboutBody};
use super::Page;


struct AboutPage{
    components: Vec<Box<dyn Element>>
}

impl AboutPage {
    fn new()->Self{
        AboutPage{ components: vec![
            Box::new(Header::new("about")),
            Box::new(AboutBody)
        ] }
    }
}

impl Page for AboutPage {
    fn title(&self)->String {
        "Axum Test | About".to_owned()
    }

    fn components(&self)->&[Box<dyn Element>] {
        return  &self.components;
    }
}

pub async fn about_renderer() -> Html<String> {
    return AboutPage::new().data();
}