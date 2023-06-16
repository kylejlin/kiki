use super::*;

#[test]
fn json() {
    let src = include_str!("../examples/json.kiki");
    let RustSrc(rust_src) = generate(src).expect("should generate Rust code");
    insta::assert_display_snapshot!(rust_src);
}

#[test]
fn kiki() {
    let src = include_str!("../examples/kiki.kiki");
    let RustSrc(rust_src) = generate(src).expect("should generate Rust code");
    insta::assert_display_snapshot!(rust_src);
}

#[test]
fn nonempty_unitlike_fieldset() {
    let src = include_str!("../examples/nonempty_unitlike_fieldset.kiki");
    let RustSrc(rust_src) = generate(src).expect("should generate Rust code");
    insta::assert_display_snapshot!(rust_src);
}

#[test]
fn balanced_parens() {
    let src = include_str!("../examples/balanced_parens.kiki");
    let RustSrc(rust_src) = generate(src).expect("should generate Rust code");
    insta::assert_display_snapshot!(rust_src);
}

#[test]
fn balanced_parens_esoteric() {
    let src = include_str!("../examples/balanced_parens_esoteric.kiki");
    let RustSrc(rust_src) = generate(src).expect("should generate Rust code");
    insta::assert_display_snapshot!(rust_src);
}

#[test]
fn balanced_parens_with_comments() {
    let src = include_str!("../examples/balanced_parens_with_comments.kiki");
    let RustSrc(rust_src) = generate(src).expect("should generate Rust code");
    insta::assert_display_snapshot!(rust_src);
}
