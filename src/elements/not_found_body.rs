use crate::utils::Utils;

use super::Element;

pub struct NotFoundBody;

impl Element for NotFoundBody{
    fn style(&self)->Option<String> {
        return Option::Some(Utils::load_style("not_found.css"));
    }

    fn layout(&self)->String {
        return Utils::load_html("not-found.html");    
    }

    fn script(&self)-> Option<String> { None }
}