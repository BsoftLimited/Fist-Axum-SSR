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
    fn style(&self)->Option<String> {
        return Option::Some(Utils::load_style("header.css"));
    }

    fn layout(&self)->String {
        if self.user.is_some(){
            return Utils::load_html("header-user.html")
            .replace("{home}", if self.active == "home" { "active" }   else { "" })
            .replace("{about}", if self.active == "about" { "active" }   else { "" })
            .replace("{url}", URL);
        }
        return Utils::load_html("header-default.html")
            .replace("{home}", if self.active == "home" { "active" }   else { "" })
            .replace("{about}", if self.active == "about" { "active" }   else { "" })
            .replace("{url}", URL);
    }

    fn script(&self)-> Option<String> { None }
}