use crate::ast::*;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "clank_grammar.pest"]
pub struct ClankParser {
    pub tree: Vec<TopLevel>,
}

impl ClankParser {
    pub fn new() -> ClankParser {
        ClankParser { tree: Vec::new() }
    }

    pub fn build_tree_parse(&mut self, file: Pairs<'_, Rule>) {
        for top_level in file.into_iter() {
            for item in top_level.into_inner() {
                match item.as_rule() {
                    Rule::func => {
                        let f = self.parse_fn(item.into_inner());
                        self.tree.push(f);
                    }
                    Rule::r#struct => {
                        let s = self.parse_struct(item.into_inner());
                        self.tree.push(s);
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
        }
    }

    pub fn parse_fn(&mut self, _f: Pairs<'_, Rule>) -> TopLevel {
        return TopLevel::Const(
            String::from("from parse_fn"),
            Box::new(Type::I32),
            Box::new(Expr::False),
        );
    }

    pub fn parse_struct(&mut self, _f: Pairs<'_, Rule>) -> TopLevel {
        return TopLevel::Const(
            String::from("from parse_struct"),
            Box::new(Type::I32),
            Box::new(Expr::False),
        );
    }
}

pub fn parse_clank(input: String) -> Vec<TopLevel> {
    let file = ClankParser::parse(Rule::program, &input)
        .expect("Unsuccessful parse")
        .next()
        .unwrap();

    let mut clank_parser = ClankParser::new();
    println!("{}", file.clone().into_inner());
    clank_parser.build_tree_parse(file.into_inner());

    return clank_parser.tree;
}
