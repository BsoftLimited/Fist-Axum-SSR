use crate::utils::Utils;

use super::Element;

pub struct Header{ active: String }

impl Header {
    pub fn new(active: &str)->Self{
        Header { active: String::from(active) }
    }
}

impl Element for Header {
    fn style(&self)->Option<String> {
        return Option::Some(Utils::load_style("header.css"));
    }

    fn layout(&self)->String {
        format!("<div  class='nav'>
                <div class='nav-brand'>Axum Alpine</div>
                <div class='nav-item {}'>Home</div>
                <div class='nav-item {}'>Signin</div>
                <div class='nav-item {}'>About</div>
            </div>", 
            if self.active == "home" { "active" }   else { "" },
            if self.active == "signin" { "active" }   else { "" },
            if self.active == "about" { "active" }   else { "" }
        )
    }

    fn script(&self)-> Option<String> { None }
}