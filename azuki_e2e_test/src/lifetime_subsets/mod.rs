mod azuki_bridge;

include!(concat!(env!("OUT_DIR"), "/lifetime_subsets.rs"));

use azuki_bridge::BorrowedPair;
use parser::non_terminals::{Document, Left, Middle, Pair, Right, Summary};
use parser::{Token, parse};

#[test]
fn lifetime_subsets() {
    let summary;
    {
        let left_source = String::from("left");
        let left;
        {
            let right_source = String::from("right");
            let pair;
            {
                let middle_source = String::from("middle");
                let document = parse_document(&left_source, &middle_source, &right_source);
                let (parsed_pair, middle, parsed_summary) = split_document(document);
                assert_eq!(Middle::Word(middle_source.as_str()), middle);
                pair = parsed_pair;
                summary = parsed_summary;
            }

            // Pair uses 'left and 'right, even though Left also consumed and
            // discarded a token borrowing the now-dropped middle source.
            let (parsed_left, right) = split_pair(pair);
            assert_eq!(right_source.as_str(), right.0);
            assert!(std::ptr::eq(right_source.as_str(), right.0));
            left = parsed_left;
        }

        // Left remains usable after the right source has been dropped.
        assert_eq!(left_source.as_str(), left.text);
        assert!(std::ptr::eq(left_source.as_str(), left.text));
    }

    // Summary has no lifetimes, although its manual reduction consumed a token
    // borrowing both the left and right sources.
    assert_eq!(Summary(String::from("left right")), summary);
}

// Independent lifetime parameters check the generated types without relying on
// inference to choose a single lifetime shared by all three sources.
fn parse_document<'left, 'middle, 'right>(
    left: &'left str,
    middle: &'middle str,
    right: &'right str,
) -> Document<'left, 'middle, 'right> {
    parse([
        Token::Left(left),
        Token::Middle(middle),
        Token::Right(right),
        Token::Middle(middle),
        Token::BorrowedPair(BorrowedPair { left, right }),
    ])
    .unwrap()
}

fn split_document<'left, 'middle, 'right>(
    document: Document<'left, 'middle, 'right>,
) -> (Pair<'left, 'right>, Middle<'middle>, Summary) {
    let Document {
        pair,
        middle,
        summary,
    } = document;
    (pair, middle, summary)
}

fn split_pair<'left, 'right>(pair: Pair<'left, 'right>) -> (Left<'left>, Right<'right>) {
    let Pair { left, right } = pair;
    (left, right)
}
