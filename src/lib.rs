pub mod lex;
pub mod parser;
pub mod assembly;
pub mod tac;

pub use crate::lex::Lex;
pub use crate::parser::parse_program;
use parser::PrettyPrint;
