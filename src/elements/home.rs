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

    fn script(&self)->String {
        return Utils::load_script("home.js");
    }

    fn layout(&self)->String { return Utils::load_html("home.html"); }

    fn match_variable(&self, name:  &str)->String {
        return match name{
            "userDetails" => {
                let mut user = "undefined".to_owned();
                if let Some(init_user) = &self.user{
                    user = format!("{{  name: {}, surname: {}, email: {} }}", init_user.name, init_user.surname, init_user.email);
                }
                return user;
            }
            __ => format!("${}", name)
        }
    }
}