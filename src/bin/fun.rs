extern crate getopts;
extern crate icndb;

use getopts::{ Matches, Options };

enum Command {
    Next(CommandOptions),
    ById(u64, CommandOptions),
    Invalid(String)
}

struct CommandOptions {
    first: Option<String>,
    last: Option<String>,
    // These may come in handy later:
    //include: Box<[String]>,
    //exclude: Box<[String]>
}


pub fn main() {
    let options = read_options();

    let response = match options {
        Command::Next(options) => icndb::next_with_names(
            &options.first.unwrap_or("Chuck".to_string()), 
            &options.last.unwrap_or("Norris".to_string())),
        Command::ById(id, options) => icndb::get_by_id_with_names(
            id, 
            &options.first.unwrap_or("Chuck".to_string()), 
            &options.last.unwrap_or("Norris".to_string())),
        Command::Invalid(desc) => {
            println!("{}", desc);
            return;
        }
    };

    match response {
        Some(result) => println!("#{}: {}", result.id, result.joke),
        None => println!("Error: not funny.")
    }
}

fn read_options() -> Command {
    let mut opts = Options::new();

    opts.optopt("f", "first", "first name", "'Bob'");
    opts.optopt("l", "last", "last name", "'Johnson'");
    opts.optopt("i", "id", "specific joke", "'373'");

    match opts.parse(std::env::args()) {
        Ok(matches) => create_command(matches),
        _ => Command::Invalid("Failed to read arguments.".to_string()) 
    }
}

/// Uses matches supplied by 
fn create_command(matches: Matches) -> Command {
    match matches.opt_str("id") {
        
        // received ID option
        Some(value) => match value.parse() {
            Ok(id) => Command::ById(id, CommandOptions {
                first: matches.opt_str("first"),
                last: matches.opt_str("last")
            }),
            _ => Command::Invalid("Could not parse ID.".to_string())
        },

        // did not recieve ID option
        None => Command::Next(CommandOptions {
            first: matches.opt_str("first"),
            last: matches.opt_str("last"),
        })
    }
}
