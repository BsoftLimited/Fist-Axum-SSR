use axum::{Router, routing::get, http::{StatusCode, Response}, response::IntoResponse, body::StreamBody, Extension};
use std::{net::SocketAddr, fs::File, io::Read};
use tower_http::services::{self, ServeDir};

mod pages;
use crate::pages::home_renderer;

mod elements;
mod utils;

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

/*async fn get_image(Extension(images_dir): Extension<String>) -> Result<impl Sized, StatusCode> {
    let serve_dir = ServeDir::new(images_dir);
    let init = serve_dir
        .oneshot("/".to_owned()) // This assumes the file path in the URL path maps to a file with the same name in the directory
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();

    return init;
}*/

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(home_renderer)).nest_service("/images", ServeDir::new("images"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);



    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
