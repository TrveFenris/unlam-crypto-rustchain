use futures::future;
use hyper::{Body, Response, StatusCode};

use super::types::ResponseFuture;

pub fn create_standard_response(body: Body, status: StatusCode) -> ResponseFuture {
    Box::new(future::ok(
        Response::builder()
            .header("Access-Control-Allow-Origin", "http://localhost:3000")
            .header("Content-Type", "application/json")
            .status(status)
            .body(body)
            .unwrap(),
    ))
}

pub fn get_transactions_new(body: Body) -> ResponseFuture {
    create_standard_response(body, StatusCode::OK)
}

pub fn get_blocks(body: Body) -> ResponseFuture {
    create_standard_response(body, StatusCode::OK)
}

pub fn get_blocks_new(body: Body) -> ResponseFuture {
    create_standard_response(body, StatusCode::OK)
}
