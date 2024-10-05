mod ast;
mod clank_parser;

use std::fs;

fn main() {
    let unparsed_file = fs::read_to_string("main.clank").expect("cannot open file.");

    clank_parser::parse_clank(unparsed_file);
}
