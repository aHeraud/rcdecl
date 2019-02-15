extern crate parser;
extern crate types;
extern crate ast;

extern crate rcdecl;

use std::env::args;

use rcdecl::describe_declarators;

fn main() {
    let mut input = args().skip(1).collect::<Vec<String>>().join(" ");
    input = input.trim_end().into();
    if input.chars().last() != Some(';') {
        input.push(';');
    }
    let output = describe_declarators(&input);
    println!("{}", output);
}
