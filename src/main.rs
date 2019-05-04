extern crate futures;
extern crate hyper;

use futures::{future, Future};

use hyper::client::HttpConnector;
use hyper::service::service_fn;
use hyper::{Body, Client, Method, Request, Response, Server, StatusCode};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type ResponseFuture = Box<Future<Item = Response<Body>, Error = GenericError> + Send>;

static TEXT: &str = "Hello, World!";
static NOTFOUND: &[u8] = b"Not Found";
static ADDRESS: &str = "127.0.0.1:1337";

fn responses(_req: Request<Body>, _client: &Client<HttpConnector>) -> ResponseFuture {
    match (_req.method(), _req.uri().path()) {
        (&Method::GET, "/") => {
            let body = Body::from(TEXT);
            let mut response = Response::builder();
            Box::new(future::ok(
                response
                    .header("Access-Control-Allow-Origin", "http://localhost:3000")
                    .header("Content-Type", "application/json")
                    .status(StatusCode::OK)
                    .body(body)
                    .unwrap(),
            ))
        }
        // POST EXAMPLE
        /*
        (&Method::POST, "/resource") => {
            api_post_response(_req) // implement an api_post_response function that gets a request as a parameter
        }
        */
        _ => {
            // Return 404 not found response.
            let body = Body::from(NOTFOUND);
            Box::new(future::ok(
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(body)
                    .unwrap(),
            ))
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
