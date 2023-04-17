use crate::utils::Utils;

use super::Element;

pub struct About;

impl Element for About {
    fn style(&self)->String {
        return Utils::load_style("about.css");
    }

    fn layout(&self)->String { return Utils::load_html("about.html"); }

    fn name(&self)->String { "about".to_lowercase() }

    fn match_variable(&self, name:  &str)->String { name.to_owned() }
}