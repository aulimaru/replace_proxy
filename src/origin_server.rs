#![allow(dead_code, unused)]
use hyper::service::{service_fn, make_service_fn};
use hyper::{Server, Body, Request, Response};
use std::net::SocketAddr;
use tokio;


async fn origin_service(req: Request<Body>) -> Result<Response<Body>, hyper::Error>{
    Ok(Response::new(Body::from("response")))
}

pub async fn start_origin_server() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc = make_service_fn(|conn| async { Ok::<_, hyper::Error>(service_fn(origin_service))});
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        println!("origin sever error: {}", e);
    }
}
