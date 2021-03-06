extern crate hyper;
extern crate hyper_router;
extern crate regex;

mod utils;

use hyper::{Body, Request, Response, Server};
use hyper::rt::{Future};
use hyper_router::{Route, RouterBuilder, RouterService};
use utils::*;

const PATH_HELLO: &str = r"/hello";
const PATH_NUM  : &str = r"/num/(?P<a>\d+)/(?P<b>\d*)";
const PATH_ROOT : &str = r"/";


fn handle_hello(_: Request<Body>) -> Response<Body> {
    let body = "Hello, World!";
    create_text_response(body, "text/plain")
}

fn handle_num(req: Request<Body>) -> Response<Body> {
    let params = capture(PATH_NUM, req.uri().path());
    let body = format!("hello: {:?}", &params);
    create_text_response(&body, "text/plain")
}

fn handle_root(_: Request<Body>) -> Response<Body> {
    create_text_response("Hyper!", "text/html")
}

// Result<T,E>: failableな処理の結果を表現する列挙型
fn routes() -> Result<RouterService, std::io::Error> {
    let router = RouterBuilder::new()
        .add(Route::get(PATH_HELLO).using(handle_hello))
        .add(Route::get(PATH_NUM).using(handle_num))
        .add(Route::get(PATH_ROOT).using(handle_root))
        .build();
    // Promiseのresolveみたいなもの？
    Ok(RouterService::new(router))
}

fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();
    println!("http://{}", addr);
    let server = Server::bind(&addr)
        .serve(routes)
        .map_err(|e| eprintln!("server error: {}", e));
    hyper::rt::run(server);
}
