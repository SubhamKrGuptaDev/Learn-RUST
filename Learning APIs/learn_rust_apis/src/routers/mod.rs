
use axum::{
    routing::{self, get},
    Router
};

pub async fn RoutersConfig() {

    let ip = "127.0.0.1";
    let port = "3000";

    let ip_address = String::from(format!("{}:{}",ip,port));

    let app = router();

    let listener = tokio::net::TcpListener::bind(&ip_address.clone())
                                .await
                                .unwrap();

    
    
}


