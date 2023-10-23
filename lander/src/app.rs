use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
pub async fn run(domain: &str, port: &u16) {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // set up domain and port from passed parameters 
    let server_details = format!("{domain}:{port}");
    let addr: SocketAddr = server_details
        .parse()
        .expect("Unable to parse socket address");

    // start listening!
    println!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html(include_str!("../../app/index.html"))
}
