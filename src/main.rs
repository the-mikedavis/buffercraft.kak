use std::io::{self, Read};
extern crate regex;
extern crate tera;
use regex::Regex;
use tera::Context;
use tera::Tera;

fn main() -> io::Result<()> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: echo '<template>' | kak-buffercraft '<pattern>' '<prospect>'");
        ::std::process::exit(1);
    }

    let pattern = Regex::new(&args[1].as_str()).unwrap_or_else(|error| {
        eprintln!("Could not compile pattern as regex: {}", error);
        ::std::process::exit(1);
    });

    let prospect = &args[2].as_str();

    let mut template = String::new();
    io::stdin().read_to_string(&mut template)?;
    let template: &str = &template[..];

    let mut tera = Tera::default();
    let mut context = Context::new();

    let matches: Vec<String> = pattern
        .find_iter(prospect)
        .map(|m| m.as_str().to_string())
        .collect();

    context.insert("matches", &matches);

    // example. this filter already exists:
    // tera.register_filter("upper", string::upper);

    tera.add_raw_template("stdin", template)
        .unwrap_or_else(|error| {
            eprintln!("Problem parsing template: {}", error);
            ::std::process::exit(1);
        });

    match tera.render("stdin", &context) {
        Ok(result) => print!("{}", result),
        Err(err) => {
            eprintln!("{}", err);
            ::std::process::exit(1);
        }
    }

    Ok(())
}
