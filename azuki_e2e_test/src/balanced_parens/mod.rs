mod azuki_bridge;

include!(concat!(env!("OUT_DIR"), "/balanced_parens.rs"));

use parser::non_terminals::Expr;
use parser::{Token, parse};

#[test]
fn balanced_parens() {
    for depth in [0, 1, 2, 8] {
        let tokens = (0..depth)
            .map(|_| Token::LParen(()))
            .chain((0..depth).map(|_| Token::RParen(())));
        let actual = parse(tokens).unwrap();
        let expected = (0..depth).fold(Expr::Empty, |inner, _| Expr::Wrap(Box::new(inner)));
        assert_eq!(expected, actual);
    }
}
