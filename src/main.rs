use std::io::{self, Read};
extern crate tera;
use tera::Context;
use tera::Tera;
// use regex::Regex;

fn main() -> io::Result<()> {
    let args: Vec<_> = std::env::args().collect();

    let mut template = String::new();
    io::stdin().read_to_string(&mut template)?;
    let template: &str = &template[..];

    let mut tera = Tera::default();
    let mut context = Context::new();

    // read matches with regex, load those into context with context.insert/1;

    // example. this filter already exists:
    // tera.register_filter("upper", string::upper);

    tera.add_raw_template("template.txt", template).unwrap_or_else(|error| {
        eprintln!("Problem parsing template: {}", error);
        ::std::process::exit(1);
    });

    match tera.render("template.txt", &context) {
      Ok(result) =>
        println!("{}", result),
      Err(err) => {
        eprintln!("Problem rendering template: {}", err);
        ::std::process::exit(1);
      }
    }

    Ok(())
}
