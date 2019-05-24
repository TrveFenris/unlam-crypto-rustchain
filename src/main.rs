#[macro_use]
extern crate lazy_static;
extern crate futures;
extern crate hyper;

mod api;
mod blockdata;
mod types;

use futures::{future, Future};

use hyper::client::HttpConnector;
use hyper::service::service_fn;
use hyper::{header, Body, Client, Method, Request, Response, Server, StatusCode};

static NOTFOUND: &[u8] = b"Not Found";
static ADDRESS: &str = "127.0.0.1:1337";

fn responses<'a>(req: Request<Body>, _client: &Client<HttpConnector>) -> types::ResponseFuture {
    println!("Received request: {:#?}", req);
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/transactions/new") => api::get_transactions_new(req),
        (&Method::OPTIONS, "/transactions/new") => Box::new(future::ok(
            Response::builder()
                .status(StatusCode::OK)
                .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .header(header::ACCESS_CONTROL_ALLOW_METHODS, "POST")
                .header(header::ACCESS_CONTROL_ALLOW_HEADERS, "content-type")
                .body(Body::from(""))
                .unwrap(),
        )),
        (&Method::GET, "/blocks") => api::get_blocks(),
        (&Method::GET, "/blocks/new") => {
            // TODO placeholder body, implement block creation
            let body = Body::from("Successfully created a new block on the rustchain.");
            api::get_blocks_new(body)
        }
        _ => api::create_standard_response(Body::from(NOTFOUND), StatusCode::NOT_FOUND),
    }
}

fn main() {
    let addr = ADDRESS.parse().unwrap();

    hyper::rt::run(future::lazy(move || {
        // Share a `Client` with all `Service`s
        let client = Client::new();

        let new_service = move || {
            // Move a clone of `client` into the `service_fn`.
            let client = client.clone();
            service_fn(move |req| responses(req, &client))
        };

        let server = Server::bind(&addr)
            .serve(new_service)
            .map_err(|e| eprintln!("Server error: {}", e));

        println!(
            "Welcome to rustchain server. Currently listening on http://{}",
            addr
        );

        server
    }));
}
