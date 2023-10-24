use axum::{
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    Router,
};
use std::net::SocketAddr;
use tower_http::{services::ServeDir, trace::TraceLayer};

#[tokio::main]
pub async fn run(domain: &str, port: &u16) {
    // build our application with a route
    // let app = Router::new().route("/", get(handler));
     
    // set up domain and port from passed parameters 
    let server_details = format!("{domain}:{port}");
    let addr: SocketAddr = server_details
        .parse()
        .expect("Unable to parse socket address");

    let app = Router::new()
    .merge(dist_router());

    // start listening!
    println!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service())
        .await
        .unwrap();
}

// router for the dist folder 
pub fn dist_router() -> Router {
    Router::new()
        .fallback_service(
            ServeDir::new("../app/dist").not_found_service(server_error.into_service()),
        )
        .layer(TraceLayer::new_for_http())
}

// when files are not reachable, handle server error
#[allow(clippy::unused_async)] 
async fn server_error() -> (StatusCode, &'static str) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong accessing static files...",
    )
}
