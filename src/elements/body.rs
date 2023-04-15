use crate::utils::{Utils, UserDetails};

use super::Element;

pub struct Body{
    user: Option<UserDetails>
}

impl Body {
    pub fn new(user: Option<UserDetails>)->Self{
        Body { user }
    }
}

impl Element for Body {
    fn style(&self)->Option<String> {
        return Option::Some(Utils::load_style("body.css"));
    }

    fn layout(&self)->String {
        Utils::load_html("home-default.html")
    }

    fn script(&self)-> Option<String> { None }
}