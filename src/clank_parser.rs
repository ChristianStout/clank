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
            match top_level.as_rule() {
                Rule::func => {
                    println!("IN build_tree_parse=>func");
                    let f = self.parse_fn(top_level.into_inner());
                    self.tree.push(f);
                }
                Rule::r#struct => {
                    println!("IN build_tree_parse=>struct");
                    let f = self.parse_struct(top_level.into_inner());
                    self.tree.push(f)
                }
                Rule::EOI => {
                    println!("IN build_tree_parse=>EOI");
                }
                _ => {
                    println!("IN build_tree_parse=>other");
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

    // for top_level in file.into_inner() {
    //     match top_level.as_rule() {
    //         Rule::func => {}
    //         Rule::r#struct => {}
    //         Rule::EOI => {}
    //         _ => {}
    //     }
    // }

    // clank_parser.build_tree_parse(file.into_inner());

    return clank_parser.tree;

    // let mut i = 0;

    // for top_level in file.into_inner() {
    //     match top_level.as_rule() {
    //         _ => {
    //             println!("{}: {:?}", i, top_level);
    //             i += 1;
    //         }
    //     }
    // }

    // println!("{:?}", file.into_inner())

    // let mut field_sum: f64 = 0.0;
    // let mut record_count: u64 = 0;

    // for record in file.into_inner() {
    //     match record.as_rule() {
    //         Rule::program => {
    //             record_count += 1;

    //             for field in record.into_inner() {
    //                 field_sum += field.as_str().parse::<f64>().unwrap();
    //             }
    //         }
    //         Rule::EOI => (),
    //         _ => unreachable!(),
    //     }
    // }

    // println!("Sum of fields: {}", field_sum);
    // println!("Number of records: {}", record_count);
}
