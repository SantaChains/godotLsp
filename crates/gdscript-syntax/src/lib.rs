//! `gdscript-syntax` —— GDScript 2.0 词法 + 语法（从零自研，engine-free，safe Rust）。
//!
//! 入口：`parse(source) -> (ast::File, Vec<ParseError>)`。
//! 设计见 `docs/from-scratch-design.md` §4–§5。

pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod token;

pub use ast::File;
pub use error::ParseError;
pub use parser::parse;
pub use token::{Token, TokenKind, keyword_kind};

/// 仅词法：`tokenize(source) -> Vec<Token>`（含结构 token Newline/Indent/Dedent）。
pub use lexer::tokenize;
