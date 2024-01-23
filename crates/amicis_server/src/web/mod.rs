use axum::Router;
use tokio::net::TcpListener;

use self::routes::{api, ui};

mod error;
mod routes;

pub struct WebServer {
    port: u16,
    listener: TcpListener,
    routes: Router,
}

impl WebServer {
    pub async fn new(port: u16) -> Self {
        let routes = Router::new()
            .nest("/", ui::routes())
            .nest("/api", api::routes());

        let listener = TcpListener::bind(("0.0.0.0", port)).await.unwrap();

        Self {
            port,
            listener,
            routes,
        }
    }

    pub async fn run(self) {
        println!("Running at {:?}", self.listener.local_addr().unwrap());
        axum::serve(self.listener, self.routes).await.unwrap()
    }
}
