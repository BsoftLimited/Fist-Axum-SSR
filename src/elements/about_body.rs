use crate::utils::Utils;

use super::Element;

pub struct AboutBody;

impl Element for AboutBody {
    fn style(&self)->Option<String> {
        return Option::Some(Utils::load_style("about.css"));
    }

    fn layout(&self)->String { return Utils::load_html("about.html"); }

    fn script(&self)-> Option<String> { None }
}