use crate::utils::Utils;

use super::Element;

pub struct Loading;

impl Element for Loading{
    fn style(&self)->String { return Utils::load_style("loading.css"); }

    fn layout(&self)->String {
        return Utils::load_html("loading.html");    
    }
}