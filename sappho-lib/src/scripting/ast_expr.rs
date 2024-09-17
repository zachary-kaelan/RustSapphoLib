use crate::scripting::ast::{AstNode, BnumType, MonadicOp, Rule, TriadicOp};
use pest::iterators::Pair;
use std::ffi::CString;

pub fn build_ast_from_expr(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::expr_monadic => {
            let mut pair = pair.into_inner();
            let op = pair.next().unwrap();
            let expr_bnum = pair.next().unwrap();
            let expr = build_ast_from_expr(expr_bnum);
            parse_monadic_op(op, expr)
        }
        Rule::expr_dyadic => {
            let mut pair = pair.into_inner();
            let lhs_pair = pair.next().unwrap();
            let lhs = build_ast_from_expr(lhs_pair);
            let op = pair.next().unwrap();
            let rhs_pair = pair.next().unwrap();
            let rhs = build_ast_from_expr(rhs_pair);
            parse_dyadic_op(op, lhs, rhs)
        }
        Rule::expr_triadic => {
            let mut pair = pair.into_inner();
            let lhs_pair = pair.next().unwrap();
            let lhs = build_ast_from_expr(lhs_pair);
            let op = pair.next().unwrap();
            let rhs_pair = pair.next().unwrap();
            let rhs = build_ast_from_expr(rhs_pair);
            let arg_pair = pair.next().unwrap();
            let arg = build_ast_from_expr(arg_pair);
            parse_triadic_op(op, lhs, rhs, arg)
        }
        Rule::float => {
            let f_str = pair.as_str();
            let (sign, f_str) = match &f_str[..1] {
                "-" => (-1f32, &f_str[1..]),
                _ => (1f32, f_str),
            };
            let float: f32 = f_str.parse().unwrap();
            AstNode::Float(sign * float)
        }
        Rule::float_optional => {
            let f_str = pair.as_str();
            let float_optional = match f_str {
                "_" => None,
                _ => Some({
                    let (sign, f_str) = match &f_str[..1] {
                        "-" => (-1f32, &f_str[1..]),
                        _ => (1f32, f_str),
                    };
                    let float: f32 = f_str.parse().unwrap();
                    sign * float
                }),
            };
            AstNode::FloatOptional(float_optional)
        }
        Rule::bnum_weight => {
            let mut pair = pair.into_inner();
            let ident = pair.next().unwrap();
            let expr_assign = pair.next().unwrap();
            let expr = build_ast_from_expr(expr_assign);
            AstNode::BnumWeight {
                ident: String::from(ident.as_str()),
                expr: Box::new(expr),
            }
        }
        Rule::bnum_assign => {
            let mut pair = pair.into_inner();
            let ident = pair.next().unwrap();
            let expr_assign = pair.next().unwrap();
            let expr = build_ast_from_expr(expr_assign);
            AstNode::BnumAssign {
                ident: String::from(ident.as_str()),
                expr: Box::new(expr),
            }
        }
        Rule::bnum_target_assign => {
            let mut pair = pair.into_inner();
            let bnum_type = pair.next().unwrap();
            let ident = pair.next().unwrap();
            let mut target: Vec<String> = vec![];
            let expr;
            loop {
                let next_pair = pair.next().unwrap();
                match next_pair.as_rule() {
                    Rule::bnum_target => target.push(String::from(next_pair.as_str())),
                    _ => {
                        expr = build_ast_from_expr(next_pair);
                        break;
                    }
                }
            }
            parse_bnum_target_assign(bnum_type, String::from(ident.as_str()), target, expr)
        }
        Rule::bnum_group => {
            let bnums: Vec<AstNode> = pair.into_inner().map(build_ast_from_expr).collect();
            AstNode::BnumGroup(bnums)
        }
        Rule::bnum_weight_group => {
            let bnums: Vec<AstNode> = pair.into_inner().map(build_ast_from_expr).collect();
            AstNode::BnumWeightGroup(bnums)
        }
        Rule::bnum_tuple => {
            let bnums: Vec<AstNode> = pair.into_inner().map(build_ast_from_expr).collect();
            AstNode::BnumTuple(bnums)
        }
        Rule::string => {
            let str = &pair.as_str();
            // Strip leading and ending quotes.
            let str = &str[1..str.len() - 1];
            // Escaped string quotes become single quotes here.
            let str = str.replace("''", "'");
            AstNode::Str(CString::new(&str[..]).unwrap())
        }
        unknown_expr => panic!("Unexpected expression: {:?}", unknown_expr),
    }
}

fn parse_monadic_op(op: Pair<Rule>, expr: AstNode) -> AstNode {
    AstNode::MonadicOp {
        op: match op.as_str() {
            "+" => MonadicOp::Positive,
            "-" => MonadicOp::Negate,
            "|" => MonadicOp::Abs,
            _ => panic!("Unsupported monadic op: {}", op.as_str()),
        },
        expr: Box::new(expr),
    }
}

fn parse_dyadic_op(op: Pair<Rule>, lhs: AstNode, rhs: AstNode) -> AstNode {
    AstNode::DyadicOp {
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
        op: match op.as_str() {
            "+" => crate::scripting::ast::DyadicOp::Plus,
            "-" => crate::scripting::ast::DyadicOp::Minus,
            "*" => crate::scripting::ast::DyadicOp::Times,
            _ => panic!("Unexpected dyadic op: {}", op.as_str()),
        },
    }
}

fn parse_triadic_op(op: Pair<Rule>, lhs: AstNode, rhs: AstNode, arg: AstNode) -> AstNode {
    AstNode::TriadicOp {
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
        arg: Box::new(arg),
        op: match op.as_str() {
            "^" => TriadicOp::Blend,
            _ => panic!("Unsupported triadic op: {}", op.as_str()),
        },
    }
}

fn parse_bnum_target_assign(
    bnum_type: pest::iterators::Pair<Rule>,
    ident: String,
    target: Vec<String>,
    expr: AstNode,
) -> AstNode {
    AstNode::BnumTargetAssign {
        ident,
        bnum_type: match bnum_type.as_str() {
            "a" => BnumType::Accordance,
            "p" => BnumType::Perception,
            "c" => BnumType::Circumstantial,
            "s" => BnumType::SelfPerception,
            "i" => BnumType::Personality,
            _ => panic!("Unsupported bnum type: {}", bnum_type.as_str()),
        },
        target: Box::new(target),
        expr: Box::new(expr),
    }
}
