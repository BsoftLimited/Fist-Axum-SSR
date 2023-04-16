use axum::response::Html;

use crate::elements::{Element, Header, HomeBody};
use super::Page;


struct HomePage{
    components: Vec<Box<dyn Element>>
}

impl HomePage {
    fn new()->Self{
        HomePage{ components: vec![
            Box::new(Header::new("home")),
            Box::new(HomeBody::new(None)),
        ] }
    }
}

impl Page for HomePage {
    fn title(&self)->String {
        "Axum Test | Home".to_owned()
    }

    fn components(&self)->&[Box<dyn Element>] {
        return  &self.components;
    }
}

pub async fn home_renderer() -> Html<String> {
    let init = HomePage::new();
    
    return init.data();
}