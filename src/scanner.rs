use std::collections::HashMap;
use tokens::Token;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
}
