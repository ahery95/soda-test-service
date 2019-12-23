#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
extern crate hyper_router;

use env_logger;
use std::env;
use std::net::ToSocketAddrs;
use url::Url;

use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::{Body, Client, Method, client::HttpConnector, Request, Response, Server,rt};
use hyper_router::{Route, RouterBuilder, RouterService};



mod cli;
mod domain;
mod middlewares;
mod proxy;
mod session;

fn main() {
  std::env::set_var("RUST_LOG", "info");
  env_logger::init();

  // The environment variables used for the Basic-Auth.
  // In the future it will be replaced by a database for the hot reload.
  let auth_user = env::var("AUTH_USER").unwrap_or_else(|_| "".to_string());
  let auth_pwd = env::var("AUTH_PWD").unwrap_or_else(|_| "".to_string());

  // Start the parsing of arguments.
  let matches = cli::init();

  // Configure addresses to listen and forward.
  let listen = matches.value_of("listen").unwrap();
  let forwarded = matches.value_of("forward").unwrap();

  // Configure the timeout for the proxy, default to 60s
  let timeout = value_t!(matches, "timeout", u32).unwrap_or(60);

  // Verify and build the forward URL.
  let forward_url = Url::parse(&format!(
    "http://{}",
    forwarded.to_socket_addrs().unwrap().next().unwrap()
  ))
  .unwrap();

  info!(
    "Server will listen on {} and forward to {}",
    listen, forward_url
  );


  fn construct_response(req: Request<Body>) -> Response<Body> {

      let body ="localhost:8081";
    Response::builder()
        .header(CONTENT_LENGTH, body.len() as u64)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct the response")
    }


    fn router_service() -> Result<RouterService, std::io::Error> {
      let router = RouterBuilder::new()
          .add(Route::get("/hello").using(construct_response))
          .add(Route::from(Method::PATCH, "/asd").using(construct_response))
          .build();

      Ok(RouterService::new(router))
  }

    let addr = "127.0.0.1:8080".parse().unwrap();
        let server = Server::bind(&addr)
          .serve(router_service)
          .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server)



  }



  // let make_service = make_service_fn(|_conn| async {
  //   Ok::<_, Infallible>(service_fn(construct_response))
  // });
  // Run the server with a state containing the forward url and the default credentials.
  // The server spawns a number of workers equals to the number of logical CPU cores,
  // each in its own thread.

//   let service = service_fn(|req: Request<Body>| async move {

//     Ok(Response::new(Body::from("Hello World")))
// });

//     let server = Server::bind(&listen)
//           .serve(&service);


//   server::new(move || {
//     let state = domain::AppState::init(
//       forward_url.clone(),
//       auth_user.clone(),
//       auth_pwd.clone(),
//       timeout,
//     );
//     App::with_state(state)
//       .middleware(Logger::default())
//       .resource("/healthcheck", |r| {
//         r.method(Method::GET).f(|_| HttpResponse::Ok())
//       })
//       .default_resource(|r| {
//         r.middleware(middlewares::Auth);
//         // r.f(proxy::forward)
//       })
//   })
//   .bind(listen)
//   .expect("Cannot bind listening port")
//   .system_exit()
//   .run();
// }
