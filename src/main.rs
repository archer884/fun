#![feature(io, plugin)]
#![plugin(regex_macros)]

extern crate hyper;
extern crate regex;

use hyper::Client;
use std::io::Read;

/// [API Documentation](http://www.icndb.com/api/)
pub fn main() {
    let content_pattern = regex!("\"[A-Z][^\"]+\"");
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

    let x = content_pattern
        .captures(&body).unwrap()
        .at(0).unwrap();

    println!("{}", &x[1..x.len() - 1]);
}

#![cfg(test)]
mod test {

}
