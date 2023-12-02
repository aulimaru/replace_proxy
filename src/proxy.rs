use hyper::server::conn::AddrIncoming;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode, Client};
use hyper_rustls::TlsAcceptor;
use std::iter;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};
use rustls::{PrivateKey, Certificate};
use rustls_pemfile::{read_one, Item, certs};


fn error(err: String) -> Error {
    Error::new(ErrorKind::Other, err)
}

fn load_certificates_from_pem(path: &str) -> std::io::Result<Vec<Certificate>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let certs = certs(&mut reader)?;

    Ok(certs.into_iter().map(Certificate).collect())
}

fn load_private_key_from_file(path: &str) -> Result<PrivateKey, Box<dyn std::error::Error>> {
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut keys = rustls_pemfile::pkcs8_private_keys(&mut reader)?;

    match keys.len() {
        0 => Err(format!("No private key found in {path}").into()),
        1 => Ok(PrivateKey(keys[0].clone())),
        _ => Err(format!("More than one private key found in {path}").into()),
    }
}


fn found(path: &str) {
    let file = File::open(&path).unwrap();
    let mut reader = BufReader::new(file);
    for item in iter::from_fn(|| read_one(&mut reader).transpose()) {
    match item.unwrap() {
        Item::X509Certificate(cert) => println!("certificate {:?}", cert),
        Item::Crl(crl) => println!("certificate revocation list: {:?}", crl),
        Item::RSAKey(key) => println!("rsa pkcs1 key {:?}", key),
        Item::PKCS8Key(key) => println!("pkcs8 key {:?}", key),
        Item::ECKey(key) => println!("sec1 ec key {:?}", key),
        _ => println!("unhandled item"),
    }
}
}

async fn request_handle(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let client = Client::new();
    println!("reqeust uri: {}", req.uri());
    //let uri = "http://localhost:3000".parse().unwrap();
    let mut forwarded_req = Request::from(req);
    //*forwarded_req.uri_mut() = uri;
    let response = client.request(forwarded_req).await;
    response
}


pub async fn start_proxy_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080".parse()?;

    let certs = load_certificates_from_pem("myCA.pem")?;
    let key = load_private_key_from_file("myCA.key")?;

    let incoming = AddrIncoming::bind(&addr)?;

    let acceptor = TlsAcceptor::builder()
        .with_single_cert(certs, key)?
        .with_all_versions_alpn()
        .with_incoming(incoming);

    let service = make_service_fn(|conn| async { Ok::<_, hyper::Error>(service_fn(request_handle))});
    let server = Server::builder(acceptor).serve(service);

    println!("start proxy");
    if let Err(e) = server.await {
        println!("proxy server error: {}", e);
    }
    Ok(())
}

pub fn check_files() {
    found("myCA.pem");
    found("myCA.key");
}
