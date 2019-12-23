use crate::domain::AppState;
use crate::session;
use bytes::Bytes;
use futures::{future, Future, Stream};
use std::time::Duration;
use hyper::{Body, Request, Response, Server};
use std::{convert::Infallible, net::SocketAddr};

pub fn forward(req: Request<(AppState)>){
  let mut new_url = req.body().forward_url.clone();
  println!("le requete est {}", new_url);
  new_url.set_path(req.uri().path());
  new_url.set_query(req.uri().query());

}

// fn construct_response(resp: ClientResponse) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
// return l;
// }

/// Inspect the given http request and then process to the stream.
/// When the body isn't empty, chunks are inspected to retrieve
/// information about the current test session.
fn inspect_and_stream(){
}
