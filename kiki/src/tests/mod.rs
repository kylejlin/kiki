use super::*;

mod get_grammar_hash_tests;

/// This module conducts end-to-end "dry run" tests.
/// A dry run is when we generate the parser,
/// but we don't actually test said generated parser.
mod e2e_dry;

mod e2e;

mod should_fail;
