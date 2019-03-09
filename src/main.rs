extern crate hyper;
extern crate hyper_router;
extern crate regex;

use hyper::{Body, Request, Response, Server};
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::rt::{Future};
use hyper_router::{Route, RouterBuilder, RouterService};
use regex::Regex;
use std::collections::HashMap;

const PATH_PATTERN_NUM: &str = r"^/num/(?P<a>\d+)/(?P<b>\d*)$";

fn captute(pattern: &str, target: &str) -> HashMap<String, String> {
    let re = Regex::new(pattern).unwrap();
    let caps = re.captures(target).unwrap();
    let cap_names = re.capture_names();

    let mut params = HashMap::new();
    cap_names
        .filter(|cap_name| cap_name.unwrap_or("").to_string().len() > 0)
        .for_each(|cap_name| {
            let name = cap_name.unwrap();
            params.insert(name.to_string(), caps[name].to_string());
        });
    params
}

fn handle_hello(_: Request<Body>) -> Response<Body> {
    let body = "Hello, World!";
    Response::builder()
        .header(CONTENT_LENGTH, body.len() as u64)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct the response")
}

fn handle_num(req: Request<Body>) -> Response<Body> {
    let params = captute(PATH_PATTERN_NUM, req.uri().path());
    let body = format!("hello: {:?}", &params);
    Response::builder()
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct the response")
}

fn handle_root(_: Request<Body>) -> Response<Body> {
    Response::builder()
        .header(CONTENT_TYPE, "text/html")
        .body(Body::from("Hyper"))
        .expect("Failed to construct the response")
}

// Result<T,E>: failableな処理の結果を表現する列挙型
fn routes() -> Result<RouterService, std::io::Error> {
    let router = RouterBuilder::new()
        .add(Route::get("/hello").using(handle_hello))
        .add(Route::get(PATH_PATTERN_NUM).using(handle_num))
        .add(Route::get("/").using(handle_root))
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