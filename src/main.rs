#![allow(unused, dead_code)]

mod proxy;
mod origin_server;
use hyper::server::conn::AddrIncoming;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper_rustls::TlsAcceptor;
use std::io;
use std::fs::File;


#[tokio::main]
async fn main() {
    tokio::spawn(origin_server::start_origin_server());
    if let Err(e) = proxy::start_proxy_server().await {
        println!("{}", e);
    }
}
