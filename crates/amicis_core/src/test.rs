use thiserror::Error;

use crate::greet;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Misc Error")]
    Misc,
}

pub type Result<T> = core::result::Result<T, Error>;

pub struct GreetRequest {
    pub name: String,
}

pub struct GreetResponse {
    pub greeting: String,
}

pub trait Server {
    fn greet(
        &self,
        request: GreetRequest,
    ) -> impl std::future::Future<Output = Result<GreetResponse>> + Send;
}

pub struct FooServer;

impl Server for FooServer {
    async fn greet(&self, request: GreetRequest) -> Result<GreetResponse> {
        let greeting = greet(request.name);

        Ok(GreetResponse { greeting })
    }
}
