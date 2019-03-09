use regex::Regex;
use std::collections::HashMap;
use hyper::{Body, Response};
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};

pub(crate) fn capture(pattern: &str, target: &str) -> HashMap<String, String> {
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

pub(crate) fn create_text_response(body: &str, content_type: &str) -> Response<Body> {
  Response::builder()
    .header(CONTENT_LENGTH, body.len() as u64)
    .header(CONTENT_TYPE, content_type)
    .body(Body::from(body.to_string()))
    .expect("Failed to construct the response")
}
