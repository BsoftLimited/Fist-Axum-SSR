use crate::utils::Utils;

use super::Element;

pub struct About;

impl Element for About {
    fn layout(&self)->String { return Utils::load_html("about.html"); }

    fn style(&self)->String {
        return Utils::load_style("about.css");
    }
}