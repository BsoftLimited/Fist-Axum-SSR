use crate::{utils::Utils, models::UserDetails};

use super::Element;

pub struct Home{
    user: Option<UserDetails>
}

impl Home {
    pub fn new(user: Option<UserDetails>)->Self{
        Home { user }
    }
}

impl Element for Home {
    fn style(&self)->String {
        return Utils::load_style("home.css");
    }

    fn layout(&self)->String { return Utils::load_html("home.html"); }

    fn name(&self)->String { "home".to_owned() }

    fn match_variable(&self, name:  &str)->String {
        return match name{
            "userDetails" => {
                let mut user = "undefined".to_owned();
                if let Some(initUser) = &self.user{
                    user = format!("{{  name: {}, surname: {}, email: {} }}", initUser.name, initUser.surname, initUser.email);
                }
                return user;
            }
            __ => name.to_owned()
        }
    }
}