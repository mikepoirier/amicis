use web::WebServer;

mod web;

#[tokio::main]
async fn main() {
    let server = WebServer::new(3000).await;

    server.run().await;
}
