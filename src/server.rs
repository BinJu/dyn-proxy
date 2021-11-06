use async_trait::async_trait;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::net::SocketAddr;
use std::{convert::Infallible, error::Error};


#[async_trait]
pub trait Service {
    async fn handle(&mut self, req: Request<Body>) -> Result<Response<Body>, Box<dyn Error + Send + Sync>>;
    async fn agent(&self, url: String) -> Result<String, Box<dyn Error + Send + Sync>>;
    fn port(&self) -> u16;
}

pub async fn make_server<S: Service + Send + Sync + Clone + 'static>(s: S) {
    let addr =SocketAddr::from(([127,0,0,1], s.port()));

    let mak_svc = make_service_fn(|_conn| {
        let s = s.clone(); 
        async move {
            Ok::<_,Infallible>(service_fn(move |req| {
                let mut s = s.clone();
                async move {
                    s.handle(req).await
                }
            }))
        }
    });

    let server = Server::bind(&addr).serve(mak_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
