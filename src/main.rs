#![feature(io, plugin)]

extern crate hyper;
extern crate "rustc-serialize" as rustc_serialize;

use hyper::Client;
use rustc_serialize::json;
use std::io::Read;

#[derive(RustcDecodable, RustcEncodable)]
struct ApiResponse {
    value: ApiResponseValue,
}

#[derive(RustcDecodable, RustcEncodable)]
struct ApiResponseValue {
    id: u64,
    joke: String,
    categories: Box<[String]>
}

/// [API Documentation](http://www.icndb.com/api/)
pub fn main() {
    let mut client = Client::new();
    let mut res = match client.get("http://api.icndb.com/jokes/random").send() {
        Ok(res) => res,
        _ => {
            println!("Funny not found.");
            return;
        }
    };

    let body = {
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        body
    };

    let api_response: ApiResponse = match json::decode(&body) {
        Ok(response) => response,
        Err(e) => panic!("{:?}", e)
    };

    println!("{}", api_response.value.joke);
}

#[cfg(test)]
mod test {

}
