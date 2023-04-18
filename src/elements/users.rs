use crate::utils::Utils;

use super::Element;

pub struct Users;

impl Element for Users {
    fn layout(&self)->String { return Utils::load_html("users.html"); }
    fn style(&self)->String {
        return Utils::load_style("users.css");
    }
    fn script(&self)->String {
        return Utils::load_script("users.js");
    }
}