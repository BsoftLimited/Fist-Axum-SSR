use crate::{utils::{URL, Utils }, models::UserDetails};

use super::Element;

pub struct Header{ active: String, user: Option<UserDetails> }

impl Header {
    pub fn new(active: &str)->Self{
        Header { active: String::from(active), user: None }
    }

    pub fn with(active: &str, user: UserDetails)->Self{
        Header{ active: active.to_owned(), user: Some(user) }
    }
}

impl Element for Header {
    fn style(&self)->String {
        return Utils::load_style("header.css");
    }

    fn layout(&self)->String {
        return  Utils::load_html("header.html");
    }

    fn name(&self)->String { "header".to_owned() }

    fn match_variable(&self, name:  &str)->String {
        return match name{
            "home" => (if self.active == "home" { "active" } else { "" }).to_owned(),
            "about" => (if self.active == "about" { "active" }   else { "" }).to_owned(),
            "url" =>  URL.to_owned(),
            __ => name.to_owned()
        }
    }
}