mod cst;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json() {
        let src = include_str!("examples/json.kiki");
        parser::FileParser::new()
            .parse(src)
            .expect("should parse correctly");
    }

    #[test]
    fn kiki() {
        let src = include_str!("examples/kiki.kiki");
        parser::FileParser::new()
            .parse(src)
            .expect("should parse correctly");
    }
}
