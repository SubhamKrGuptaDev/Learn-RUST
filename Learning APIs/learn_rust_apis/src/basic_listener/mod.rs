
use axum::{
    routing::get,
    Router
};

pub async fn basic_api() {
    let ip = "0.0.0.0";
    let port = "3000";

    let ip_address = String::from(format!("{}:{}",ip,port));

    let app = router();

    let listener = tokio::net::TcpListener::bind(ip_address.clone())
            .await
            .unwrap();

    println!("Server Start Listening on {}...",ip_address);

    // Serve Started and Continue Listener
    axum::serve(listener, app).await.unwrap();
}

fn router() -> Router {
    println!("API Called");
    Router::new().route("/api", get( get_hello ))
}

async fn get_hello() -> &'static str {
    println!("Get API Called: ");
    "Hello Subham"
}

// fn get_hello(&str: name) -> &str {
//     "Hello "+name
// }
