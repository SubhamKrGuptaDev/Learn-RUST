mod basic_listener;
mod routers;

#[tokio::main]
async  fn main() {
    basic_listener::basic_api().await;
    routers::RoutersConfig().await;
}
