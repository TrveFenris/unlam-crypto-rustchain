use futures::Future;
use hyper::{Body, Response};

pub type GenericError = Box<dyn std::error::Error + Send + Sync>;
pub type ResponseFuture = Box<Future<Item = Response<Body>, Error = GenericError> + Send>;
