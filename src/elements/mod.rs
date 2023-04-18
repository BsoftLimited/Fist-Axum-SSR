use regex::Regex;

mod header;
use std::{collections::HashMap};

pub use header::Header;

mod home;
pub use home::Home;

mod not_found;
pub use not_found::NotFound;

mod about;
pub use about::About;

mod users;
pub use users::Users;

mod loading;
pub use loading::Loading;

use crate::models::UserDetails;

pub struct ElementLayout{
    pub layout: String, pub style: String, pub script: String
}

pub trait Element{
    fn layout(&self)->String;
    fn style(&self)->String{ String::new() }
    fn script(&self)->String { String::new() }
    fn match_variable(&self, name:  &str)->String { format!("${}", name) }
    fn html(&self)-> ElementLayout{
        let mut layout = self.layout();
        let mut style = self.style();
        let mut script = self.script();


        for variable in find_variables(layout.as_str()){
            let name = variable.replace("$", "");
            layout = layout.replace(variable.as_str(), self.match_variable(name.as_str()).as_str());
        }

        let (elements, options) = find_elements(layout.as_str());
        for (raw, element) in elements{
            if let Some(value) = register(element.clone(), options.get(element.as_str()).unwrap()){
                let init = value.html();
                style.push_str(&init.style);
                script.push_str(&init.script);

                layout = layout.replace(&raw, &init.layout);
            }
        }
        return ElementLayout{ layout, style, script };
    }
}

fn find_elements(s: &str) -> (Vec<(String, String)>, HashMap<String, HashMap<String, String>>) {
    let re = Regex::new(r#"(\{(\w+)(\((\s*\w+\s*:\s*(?:'[^']*'|"[^"]*"|\w+)(,\s*\w+\s*:\s*(?:'[^']*'|"[^"]*"|\w+))*\s*)\))?\})"#).unwrap();
    let mut variables = Vec::new();
    let mut variable_options = HashMap::new();
    for capture in re.captures_iter(s) {
        let raw_str = capture[1].to_string();
        let name = capture[2].to_string();
        let options_str = capture.get(4).map_or("", |m| m.as_str());
        let options_re = Regex::new(r#"\s*(\w+)\s*:\s*(?:'([^']*)'|"([^"]*)"|(\w+))"#).unwrap();
        let options = options_re
            .captures_iter(options_str)
            .map(|cap| {
                let key = cap[1].to_string();
                let value = cap.get(2).or(cap.get(3)).or(cap.get(4)).unwrap().as_str().to_string();
                (key, value)
            })
            .collect();
        variables.push((raw_str, name.clone()));
        variable_options.insert(name, options);
    }
    (variables, variable_options)
}

fn find_variables(text: &str) -> Vec<String> {
    let re = Regex::new(r#"(?P<variable>\$\w+)"#).unwrap();
    re.captures_iter(text)
        .map(|cap| cap.name("variable").unwrap().as_str().to_string())
        .collect()
}

fn unwrap_string(params: &HashMap<String, String>, key: &str)->String{
    let value = params.get(key);
    if let Some(init) = value{
       return  init.clone();
    }
    return "".to_owned();
}

fn register(name: String, option: &HashMap<String, String>)->Option<Box<dyn Element>>{
    match name.as_str(){
        "about" => {
            return Some(Box::new(About)); 
        }
        "header" => {        
            return Some(Box::new(Header::new(unwrap_string(&option, "active").as_str()))); 
        },
        "home" =>{
            let name = unwrap_string(&option, "name");
            let surname = unwrap_string(&option, "surname");
            let email = unwrap_string(&option, "email");

            let mut user: Option<UserDetails> = None;
            if !name.is_empty() && !surname.is_empty() && !email.is_empty(){
                user = Some(UserDetails { name, surname, email });
            }
            return Some(Box::new(Home::new(user)));
        },
        "loading" =>{
            return Some(Box::new(Loading));
        }
        __ => None
    }
}