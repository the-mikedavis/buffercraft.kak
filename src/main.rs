use std::io::{self, Read};
extern crate tera;
use tera::Context;
use tera::Tera;

fn main() -> io::Result<()> {
    let args: Vec<_> = std::env::args().collect();

    let mut template = String::new();
    io::stdin().read_to_string(&mut template)?;

    let mut tera = Tera::default();
    let mut context = Context.new();

    // read matches with regex, load those into context with context.insert/1;

    // example. this filter already exists:
    // tera.register_filter("upper", string::upper);

    tera.add_raw_template("template.txt", template)?;

    let result = Tera::one_off(template, context, false);

    println!("{}", result);

    Ok(())
}
