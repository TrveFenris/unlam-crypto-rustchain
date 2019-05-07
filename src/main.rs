extern crate futures;
extern crate hyper;

mod api;
mod types;

use futures::{future, Future};

use hyper::client::HttpConnector;
use hyper::service::service_fn;
use hyper::{Body, Client, Method, Request, Server, StatusCode};

static TEXT: &str = "Hello, World!";
static NOTFOUND: &[u8] = b"Not Found";
static ADDRESS: &str = "127.0.0.1:1337";

fn responses(_req: Request<Body>, _client: &Client<HttpConnector>) -> types::ResponseFuture {
    match (_req.method(), _req.uri().path()) {
        (&Method::GET, "/") => {
            // Hello world test method
            api::create_standard_response(Body::from(TEXT), StatusCode::OK)
        }
        (&Method::POST, "/transactions/new") => {
            // TODO placeholder body, implement transaction creation
            let body = Body::from("Successfully added a new transaction to the rustchain.");
            api::get_transactions_new(body)
        }
        (&Method::GET, "/blocks") => {
            // TODO placeholder body, implement a getter for the complete chain
            let body = Body::from("Getting all the blocks on the rustchain...");
            api::get_blocks(body)
        }
        (&Method::GET, "/blocks/new") => {
            // TODO placeholder body, implement block creation
            let body = Body::from("Successfully created a new block on the rustchain.");
            api::get_blocks_new(body)
        }
        _ => {
            // Return 404 not found response.
            api::create_standard_response(Body::from(NOTFOUND), StatusCode::NOT_FOUND)
        }
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
