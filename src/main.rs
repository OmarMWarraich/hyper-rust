use std::{convert::Infallible,net::SocketAddr};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Method, StatusCode};
use futures::TryStreamExt as _;


// #[tokio::main]
// async fn main() {
//     // We'll bind to 127.0.0.1:3000
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

//     // A `Service` is needed for every connection, so this
//     // creates one from our `hello_world` function.
//     let make_svc = make_service_fn(|_conn| async {
//         // service_fn converts our function into a `Service`
//         Ok::<_, Infallible>(service_fn(hello_world))
//     });

//     let server = Server::bind(&addr).serve(make_svc);

//     // Run this server for... forever!
//     if let Err(e) = server.await {
//         eprintln!("server error: {}", e);
//     }
// }

// async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
//     Ok(Response::new("Hello, World".into()))
// }


#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3003
    let addr = SocketAddr::from(([127, 0, 0, 1], 3003));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    
let mut response = Response::new(Body::empty());

match (req.method(), req.uri().path()) {
    (&Method::GET, "/") => {
        *response.body_mut() = Body::from("Try POSTing data to /echo");
    },
    (&Method::POST, "/echo") => {
        *response.body_mut() = req.into_body();
    },
    (&Method::POST, "/echo/kuchbhi") => {
        // This is actually a new `futures::Stream`...
    let mapping = req
    .into_body()
    .map_ok(|chunk| {
        chunk.iter()
            .map(|byte| byte.to_ascii_uppercase())
            .collect::<Vec<u8>>()
    });

// Use `Body::wrap_stream` to convert it to a `Body`...
*response.body_mut() = Body::wrap_stream(mapping);
},

    _ => {
        *response.status_mut() = StatusCode::NOT_FOUND;
        *response.body_mut() = Body::from("Requested resource not found at this endpoint")
    },
};

Ok(response)
}