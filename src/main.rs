use axum::{Router, routing::get, error_handling::HandleErrorLayer, http::StatusCode};
use std::{net::SocketAddr, time::Duration};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tower::{BoxError, ServiceBuilder};

mod pages;
use crate::{models::{user_details, user_create}, pages::{home_page, about_page, users_page, not_found_page}};

mod config;
mod elements;
mod utils;
mod models;


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
    // compose the routes
    let app = Router::new()
        .route("/api/user", get(user_details).post(user_create))
        .route("/", get(home_page))
        .route("/about", get(about_page))
        .route("/users", get(users_page))
        .nest_service("/images", ServeDir::new("images"))
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>(){
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner()
    );
 
    // add a fallback service for handling routes to unknown paths
    let app = app.fallback(not_found_page);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);



    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
