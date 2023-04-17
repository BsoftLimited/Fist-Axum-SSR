use crate::utils::{URL, Utils };

use super::Element;

pub struct Header{ active: String }

impl Header {
    pub fn new(active: &str)->Self{
        Header { active: String::from(active)}
    }
}


impl Element for Header {
    fn style(&self)->String {
        return Utils::load_style("header.css");
    }

    fn layout(&self)->String {
        return  Utils::load_html("header.html");
    }

    fn match_variable(&self, name:  &str)->String {
        return match name{
            "home" => (if self.active == "home" { "active" } else { "" }).to_owned(),
            "users" => (if self.active == "users" { "active" }   else { "" }).to_owned(),
            "about" => (if self.active == "about" { "active" }   else { "" }).to_owned(),
            "url" =>  URL.to_owned(),
            __ => name.to_owned()
        }
    }
}