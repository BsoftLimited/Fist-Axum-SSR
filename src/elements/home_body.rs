use crate::{utils::Utils, models::UserDetails};

use super::Element;

pub struct HomeBody{
    user: Option<UserDetails>
}

impl HomeBody {
    pub fn new(user: Option<UserDetails>)->Self{
        HomeBody { user }
    }
}

impl Element for HomeBody {
    fn style(&self)->Option<String> {
        return Option::Some(Utils::load_style("home.css"));
    }

    fn layout(&self)->String {
        if self.user.is_some(){
            return Utils::load_html("home-user.html");    
        }
        Utils::load_html("home-default.html")
    }

    fn script(&self)-> Option<String> { None }
}