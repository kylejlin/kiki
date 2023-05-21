mod cst;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub calc1); // synthesized by LALRPOP

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json() {
        let src = include_str!("examples/json.kiki");
        calc1::FileParser::new()
            .parse(src)
            .expect("should parse correctly");
    }

    #[test]
    fn kiki() {
        let src = include_str!("examples/kiki.kiki");
        calc1::FileParser::new()
            .parse(src)
            .expect("should parse correctly");
    }
}
