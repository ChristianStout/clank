use crate::ast::*;
use pest::iterators::{Pair, Pairs};
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
                    Rule::r#const => {
                        let c = self.parse_const(item.into_inner());
                        self.tree.push(c);
                    }
                    _ => {
                        println!("`{}` was unreachable in build_parse_tree", item.as_str());
                        unreachable!();
                    }
                }
            }
        }
    }

    pub fn parse_fn(&self, p: Pairs<'_, Rule>) -> TopLevel {
        let pairs: Vec<Pair<'_, Rule>> = p.collect();
        
        let id = pairs[0].as_str().to_string();
        
        let mut index = 1;
        let mut parameters = vec![];

        if let Rule::params = pairs[index].as_rule() {
            parameters = self.parse_params(pairs[index].clone().into_inner());
            index += 1;
        }

        let t = Box::new(self.get_type(pairs[index].as_str()));

        let stmts = self.parse_stmt_many(pairs[index].clone().into_inner()); // TODO: revome clone

        return TopLevel::Fn(id, parameters, t, stmts); // TODO: Add parameters
    }

    pub fn parse_params(&self, p: Pairs<'_, Rule>) -> Vec<(String, Type)> {
        let mut parameters = Vec::new();
        let pairs: Vec<Pair<'_, Rule>> = p.collect();

        // pairs looks like [id, type, id, type, id, type, ...]
        let mut index = 0;

        while index < pairs.len() {
            let id = String::from(pairs[index].as_str());
            index += 1;
            let t = self.get_type(pairs[index].as_str());
            index += 1;

            parameters.push((id, t));
        }

        return parameters;
    }

    pub fn parse_struct(&mut self, _p: Pairs<'_, Rule>) -> TopLevel {
        return TopLevel::Const(
            String::from("from parse_struct"),
            Box::new(Type::I32),
            Box::new(Expr::False),
        );
    }

    pub fn parse_const(&mut self, p: Pairs<'_, Rule>) -> TopLevel {
        let mut pairs: Vec<Pair<'_, Rule>> = p.collect();

        let id = pairs[0].as_str().to_string();
        let t = Box::new(self.get_type(pairs[1].as_str()));
        let expr = Box::new(
            self.parse_expr(
                pairs
                    .pop()
                    .expect("Somehow, the const was empty")
                    .into_inner(),
            ),
        );

        return TopLevel::Const(id, t, expr);
    }

    pub fn parse_unary(&self, p: Pair<'_, Rule>) -> Expr {
        let symbol: char = p.as_str().chars().collect::<Vec<char>>()[0];

        let expr = Box::new(self.parse_expr(p.into_inner()));
        println!("{}", symbol);
        match symbol {
            '+' => { return Expr::UnaryPos(expr); }
            '-' => { return Expr::UnaryNeg(expr); }
            '!' => { return Expr::Not(expr); }
            _   => { return Expr::Str("PARSE_UNARY_FAILED".to_string()); }
        }
    }

    pub fn parse_expr(&self, p: Pairs<'_, Rule>) -> Expr {
        let mut expr: Expr = Expr::False;
        for pair in p.into_iter() {
            match pair.as_rule() {
                Rule::id => expr = Expr::Id(pair.as_str().to_string()),
                Rule::num => {
                    expr = Expr::Num(
                        pair.as_str()
                            .parse::<i32>()
                            .expect("Somehow, a number was parsed, but it isn't a number"),
                    )
                }
                Rule::string => {
                    expr = Expr::Str(pair.into_inner().next().unwrap().as_str().to_string())
                }
                Rule::unary_e => {
                    println!("from expr: {}", pair.as_str());
                    expr = self.parse_unary(pair)
                }
                //Rule::id => { expr = Expr::Id(pair.as_str().to_string()) },
                //Rule::num => { expr = Expr::Num(pair.as_str().parse::<i32>().expect("Somehow, a number was parsed, but it isn't a number")) }
                //Rule::string => { expr = Expr::Str(pair.into_inner().next().unwrap().as_str().to_string()) }
                Rule::char => {
                    let char_vec: Vec<char> =
                        pair.into_inner().next().unwrap().as_str().chars().collect();

                    assert!(char_vec.len() == 1 || char_vec.len() == 2 && char_vec[0] == '\\');
                    println!("{:?}", char_vec);
                    expr = Expr::Chr(char_vec[0]);
                }
                Rule::expr => {
                    return self.parse_expr(pair.into_inner()); // very hacky, make sure to otimize CALLS. Should remove
                }
                _ => {
                    println!("`{}` was unreachable in parse_expr", pair.as_str());
                    println!("Token: {:?}", pair);
                    unreachable!();
                }
            }
        }

        return expr;
    }

    pub fn parse_stmt(&self, stmt: Pair<'_, Rule>) -> Stmt {
        println!("\n\n{:?}", stmt);
        match stmt.as_rule() {
            Rule::expr_stmt => {
                return Stmt::Expr(Box::new(self.parse_expr(stmt.into_inner())));
            }
            Rule::ret_stmt => {
                return Stmt::Return(Box::new(self.parse_expr(stmt.into_inner())));
            }
            _ => {}
        }
        return Stmt::Expr(Box::new(Expr::False));
    }

    pub fn parse_stmt_many(&self, stmts: Pairs<'_, Rule>) -> Vec<Stmt> {
        let mut ret_vec: Vec<Stmt> = vec![];
        println!("from parse_stmts_many: {:?}", stmts);
        for stmt in stmts.into_iter() {
            ret_vec.push(self.parse_stmt(stmt));

            println!("\n{:?}", ret_vec);
        }
        return ret_vec;
    }

    pub fn get_type(&self, s: &str) -> Type {
        match s {
            "i32" => Type::I32,
            "u8" => Type::U8,
            "string" => Type::String,
            "char" => Type::Char,
            "bool" => Type::Bool,
            _ => Type::Custom(s.to_string()), // TODO: add Array() and Fn()
        }
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
