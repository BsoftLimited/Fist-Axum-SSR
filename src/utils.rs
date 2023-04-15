use std::{fs::File, io::Read};
pub struct Utils;

impl Utils{
    pub fn load_file(path: &str)->String{
        match &mut File::open(path){
            Ok(file) =>{
                let mut init = String::new();
                if let Err(error) = file.read_to_string(&mut init){
                    panic!("{}", error);
                }
                return init;
            },
            Err(error) =>{ panic!("{}", error); }
        }
    }

    pub fn load_style(file: &str)->String{
        let init = format!("css/{}", file);
        return  Utils::load_file(init.as_str());
    }

    pub fn load_html(file: &str)->String{
        let init = format!("htmls/{}", file);
        return  Utils::load_file(init.as_str());
    }

    pub fn load_script(file: &str)->String{
        let init = format!("js/{}", file);
        return  Utils::load_file(init.as_str());
    }
}

pub struct UserDetails{
    name: String,
    surname: String,
    email: String,
}