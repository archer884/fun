extern crate icndb;

pub fn main() {
    match icndb::next() {
        Some(response) => println!("#{}: {}", response.id, response.joke),
        None => println!("Error: not funny.")
    }
}
