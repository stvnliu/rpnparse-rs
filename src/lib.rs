//! Reverse Polish Notation parsing library
//!
//! Example code:
//! ```
//! use rpn::eval;
//! use std::collections::HashMap;
//! let mut m: HashMap<char, f32> = HashMap::new();
//! m.insert('a', 1.0);
//! m.insert('b', 2.0);
//! m.insert('c', 3.0);
//! println!("{}", match eval("ab-c+c*", m) {
//!     Ok(n) => n.to_string(),
//!     Err(s) => s,
//! })
//! ```
//!
//! TODOs
//! - Binary tree converting from infix to postfix operation

mod rpn;
mod rpntree;
use std::collections::HashMap;

pub use rpn::{eval, RpnOperation};
pub use rpntree::eval_infix;
pub fn test() {
    let mut m: HashMap<char, f32> = HashMap::new();
    m.insert('a', 1.0);
    m.insert('b', 2.0);
    m.insert('c', 3.0);
    println!(
        "{}",
        match eval("ab-c+c*", m) {
            Ok(n) => n.to_string(),
            Err(s) => s,
        }
    )
}
