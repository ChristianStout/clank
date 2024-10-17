mod ast;
mod clank_parser;

use std::fs;

fn main() {
    let unparsed_file =
        fs::read_to_string("/home/lambda/Documents/projects/clank/main.clank")
            .expect("cannot open file.");

    let tree = clank_parser::parse_clank(unparsed_file);

    println!("{:?}", tree);
}
