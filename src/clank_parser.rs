use crate::ast::*;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "clank_grammar.pest"]
pub struct ClankParser;

pub fn parse_clank(input: String) -> Vec<TopLevel> {
    let file = ClankParser::parse(Rule::program, &input)
        .expect("Unsuccessful parse")
        .next()
        .unwrap();

    let top = vec![];

    println!("{}", file.clone().into_inner());

    for node in file.into_inner() {
        match node.as_rule() {
            Rule::func => {}
            Rule::r#struct => {}
            Rule::EOI => {}
            _ => {}
        }
    }

    return top;

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
