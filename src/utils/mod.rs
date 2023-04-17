use std::{fs::File, io::Read};

pub const URL: &str = "http://localhost:3000";
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

/*async fn serve_image() -> Response<axum::body::Body> {
    let mut file = File::open("images/quitting.svg").unwrap_or_else(|_| panic!("Could not open image file"));

    let mut buffer = Vec::new();
    let result = file.read_to_end(&mut buffer);

    match result {
        Ok(_) => {
            let response = Response::builder()
                .status(200)
                .header("content-type", "image/svg")
                .header("content-length", buffer.len())
                .header(
                    "content-disposition",
                    "attachment; filename=quitting.svg",
                )
                .body(axum::body::Body::from(buffer))
                .unwrap();

            response
        }
        Err(e) => {
            println!("Failed to read image file: {}", e);
            Response::builder().status(500).body(axum::body::Body::empty()).unwrap()
        }
    }
}*/