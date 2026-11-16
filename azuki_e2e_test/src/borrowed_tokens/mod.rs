mod azuki_bridge;

include!(concat!(env!("OUT_DIR"), "/borrowed_tokens.rs"));

use parser::non_terminals::Document;
use parser::{Token, parse};

#[test]
fn borrowed_tokens() {
    for text in ["", "azuki", "azuki parser generator"] {
        let source = text.to_owned();
        let words: Vec<_> = source.split_whitespace().collect();
        let actual = parse(words.iter().copied().map(Token::Word)).unwrap();
        let expected = Document {
            words: words.clone(),
        };
        assert_eq!(expected, actual);

        for (expected, actual) in words.iter().zip(&actual.words) {
            assert!(std::ptr::eq(*expected, *actual));
        }
    }
}
