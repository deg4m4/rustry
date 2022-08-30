use hyper::{Body, Request, Response};
use std::convert::Infallible;

pub async fn main_handler(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}