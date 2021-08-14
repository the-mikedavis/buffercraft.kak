use std::collections::HashMap;
use std::io::{self, Read};
extern crate inflector;
extern crate regex;
extern crate tera;
use inflector::Inflector;
use regex::Regex;
use tera::{to_value, try_get_value, Context, Result as TeraResult, Tera, Value};

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

    let matches: Vec<String> = pattern
        .captures(prospect)
        .unwrap_or_else(|| {
            eprintln!("Could not find prospect matches in pattern");
            ::std::process::exit(1);
        })
        .iter()
        .map(|m| m.unwrap().as_str().to_string())
        .collect();

    let mut tera = Tera::default();
    tera.register_filter("dot", dot);
    tera.register_filter("camelcase", camelcase);
    tera.register_filter("pascalcase", pascalcase);

    let mut context = Context::new();

    context.insert("matches", &matches);

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

fn dot(value: &Value, _args: &HashMap<String, Value>) -> TeraResult<Value> {
    let s = try_get_value!("dot", "value", String, value);
    Ok(to_value(s.replace("/", ".")).unwrap())
}

fn camelcase(value: &Value, _args: &HashMap<String, Value>) -> TeraResult<Value> {
    let s = try_get_value!("camelcase", "value", String, value);
    let camel = s
        .split("/")
        .map(|s| s.to_camel_case())
        .collect::<Vec<String>>()
        .join("/");
    Ok(to_value(camel).unwrap())
}

fn pascalcase(value: &Value, _args: &HashMap<String, Value>) -> TeraResult<Value> {
    let s = try_get_value!("pascalcase", "value", String, value);
    let pascal = s
        .split("/")
        .map(|s| s.to_pascal_case())
        .collect::<Vec<String>>()
        .join("/");
    Ok(to_value(pascal).unwrap())
}
