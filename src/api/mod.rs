use futures::prelude::*;
use futures::{future, Future};
use hyper::http::Request;
use hyper::{header, Body, Response, StatusCode};
use serde_json::Result as SerdeResult;
use std::sync::Mutex;

use super::types::ResponseFuture;

use super::blockdata::blockchain::Blockchain;
use super::blockdata::transaction::Transaction;

lazy_static! {
    pub static ref RUSTCHAIN: Mutex<Blockchain> = Mutex::new(Blockchain::genesis());
}

pub fn create_standard_response(body: Body, status: StatusCode) -> ResponseFuture {
    Box::new(future::ok(
        Response::builder()
            .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:3000")
            .header(header::CONTENT_TYPE, "application/json")
            .status(status)
            .body(body)
            .unwrap(),
    ))
}

pub fn get_transactions_new(req: Request<Body>) -> ResponseFuture {
    Box::new(req.into_body().concat2().from_err().and_then(|body| {
        // TODO: Replace all unwraps with proper error handling
        let str = String::from_utf8(body.to_vec())?;
        let data: SerdeResult<Transaction> = serde_json::from_str(&str);
        let response;
        match data {
            Ok(tx) => {
                println!("TRANSACTION RECEIVED: {:#?}", tx);
                RUSTCHAIN.lock().unwrap().add_transaction(tx);
                println!(
                    "AFTER ADDING A TRANSACTION, THE RUSTCHAIN IS NOW: {:#?}",
                    *RUSTCHAIN
                );
                response = Response::builder()
                    .status(StatusCode::OK)
                    .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                    .header("Content-Type", "application/json")
                    .body(Body::from("Transaction created succesfully."))?;
            }
            Err(_e) => {
                println!("BAD TRANSACTION!");
                response = Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                    .header("Content-Type", "application/json")
                    .body(Body::from("Error while creating the Transaction."))?;
            }
        }
        Ok(response)
    }))
}

pub fn get_blocks() -> ResponseFuture {
    let blocks = RUSTCHAIN.lock().unwrap().get_blocks();
    let json = serde_json::to_string(&blocks);
    let response;
    match json {
        Ok(json_string) => {
            println!("Requested blocks");
            response = Response::builder()
                .status(StatusCode::OK)
                .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .header("Content-Type", "application/json")
                .body(Body::from(json_string));
        }
        Err(_e) => {
            println!("BAD BLOCKS REQUEST!");
            response = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .header("Content-Type", "application/json")
                .body(Body::from("Error while getting the Rustchain blocks."));
        }
    }
    Box::new(future::ok(response.unwrap()))
}

pub fn get_blocks_new() -> ResponseFuture {
    let last_block = RUSTCHAIN.lock().unwrap().get_last_block();
    let last_proof = last_block.proof;
    let proof = RUSTCHAIN.lock().unwrap().proof_of_work(last_proof);
    // We must receive a reward for finding the proof.
    // The sender is "0" to signify that this node has mined a new coin.
    RUSTCHAIN.lock().unwrap().add_transaction(Transaction {
        sender: String::from("0"),
        recipient: String::from("TBD"), // TODO get the recipient address?
        amount: 1,
    });
    let new_block = RUSTCHAIN.lock().unwrap().create_block(proof);
    println!(
        "AFTER CREATING A NEW BLOCK, THE RUSTCHAIN IS NOW: {:#?}",
        *RUSTCHAIN
    );
    let json = serde_json::to_string(&new_block);
    let response;
    match json {
        Ok(json_string) => {
            println!("Created a new block");
            response = Response::builder()
                .status(StatusCode::OK)
                .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .header("Content-Type", "application/json")
                .body(Body::from(json_string));
        }
        Err(_e) => {
            println!("ERROR CREATING BLOCK!");
            response = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    "Error while creating a new block on the Rustchain.",
                ));
        }
    }
    Box::new(future::ok(response.unwrap()))
}
