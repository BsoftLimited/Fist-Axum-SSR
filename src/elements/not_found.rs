use crate::utils::Utils;

use super::Element;

pub struct NotFound;

impl Element for NotFound{
    fn style(&self)->String { return Utils::load_style("not_found.css"); }

    fn layout(&self)->String {
        return Utils::load_html("not-found.html");    
    }

    fn match_variable(&self, name:  &str)->String {
        return name.to_owned();
    }
}