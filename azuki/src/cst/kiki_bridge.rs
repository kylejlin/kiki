use super::*;

use raw_parser::Token as KikiToken;

pub(crate) struct TaggedSpan<T> {
    /// Inclusive.
    pub start: TextPosition,

    /// Exclusive.
    pub end: TextPosition,

    pub _ntt: Ntt<T>,
}
impl<T> Debug for TaggedSpan<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("TaggedSpan")
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}
impl<T> Clone for TaggedSpan<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for TaggedSpan<T> {}
impl<T> PartialEq for TaggedSpan<T> {
    fn eq(&self, other: &Self) -> bool {
        let Self { start, end, _ntt } = self;
        *start == other.start && *end == other.end
    }
}
impl<T> Eq for TaggedSpan<T> {}
impl<T> Hash for TaggedSpan<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Self { start, end, _ntt } = self;
        start.hash(state);
        end.hash(state);
    }
}

pub(crate) mod span_of {
    use super::*;

    use token::tagged_token::ntt;

    pub type StartKw = TaggedSpan<ntt::StartKw>;
    pub type StructKw = TaggedSpan<ntt::StructKw>;
    pub type EnumKw = TaggedSpan<ntt::EnumKw>;
    pub type ManualKw = TaggedSpan<ntt::ManualKw>;
    pub type TerminalKw = TaggedSpan<ntt::TerminalKw>;
    pub type BridgeKw = TaggedSpan<ntt::BridgeKw>;
    pub type UpperSelfKw = TaggedSpan<ntt::UpperSelfKw>;

    pub type Underscore = TaggedSpan<ntt::Underscore>;
    pub type LParen = TaggedSpan<ntt::LParen>;
    pub type RParen = TaggedSpan<ntt::RParen>;
    pub type LSquare = TaggedSpan<ntt::LSquare>;
    pub type RSquare = TaggedSpan<ntt::RSquare>;
    pub type LCurly = TaggedSpan<ntt::LCurly>;
    pub type RCurly = TaggedSpan<ntt::RCurly>;
    pub type Comma = TaggedSpan<ntt::Comma>;
    pub type DoubleColon = TaggedSpan<ntt::DoubleColon>;
    pub type Colon = TaggedSpan<ntt::Colon>;
    pub type Equal = TaggedSpan<ntt::Equal>;
    pub type Octothorpe = TaggedSpan<ntt::Octothorpe>;

    pub type UpperName = TaggedSpan<ntt::UpperName>;
    pub type LowerName = TaggedSpan<ntt::LowerName>;
    pub type DollarUpperName = TaggedSpan<ntt::DollarUpperName>;
    pub type AtUpperName = TaggedSpan<ntt::AtUpperName>;
    pub type AtLowerName = TaggedSpan<ntt::AtLowerName>;
    pub type SingleQuoteLowerName = TaggedSpan<ntt::SingleQuoteLowerName>;

    pub type RustStringLiteral = TaggedSpan<ntt::RustStringLiteral>;
}

impl KikiToken {
    pub(crate) fn from_lexer_token(token: Token, file: FileSource) -> Self {
        macro_rules! impl_ {
            ($t:ident, $Variant:ident) => {{
                let r = file.substr_range($t.raw).unwrap();

                KikiToken::$Variant(TaggedSpan {
                    start: TextPosition {
                        byte_index: r.start,
                    },
                    end: TextPosition { byte_index: r.end },
                    _ntt: Ntt::new(),
                })
            }};
        }
        match token {
            Token::StartKw(t) => impl_!(t, StartKw),
            Token::StructKw(t) => impl_!(t, StructKw),
            Token::EnumKw(t) => impl_!(t, EnumKw),
            Token::ManualKw(t) => impl_!(t, ManualKw),
            Token::TerminalKw(t) => impl_!(t, TerminalKw),
            Token::BridgeKw(t) => impl_!(t, BridgeKw),
            Token::UpperSelfKw(t) => impl_!(t, UpperSelfKw),
            Token::Underscore(t) => impl_!(t, Underscore),
            Token::LParen(t) => impl_!(t, LParen),
            Token::RParen(t) => impl_!(t, RParen),
            Token::LSquare(t) => impl_!(t, LSquare),
            Token::RSquare(t) => impl_!(t, RSquare),
            Token::LCurly(t) => impl_!(t, LCurly),
            Token::RCurly(t) => impl_!(t, RCurly),
            Token::Comma(t) => impl_!(t, Comma),
            Token::DoubleColon(t) => impl_!(t, DoubleColon),
            Token::Colon(t) => impl_!(t, Colon),
            Token::Equal(t) => impl_!(t, Equal),
            Token::Octothorpe(t) => impl_!(t, Octothorpe),
            Token::UpperName(t) => impl_!(t, UpperName),
            Token::LowerName(t) => impl_!(t, LowerName),
            Token::DollarUpperName(t) => impl_!(t, DollarUpperName),
            Token::AtUpperName(t) => impl_!(t, AtUpperName),
            Token::AtLowerName(t) => impl_!(t, AtLowerName),
            Token::SingleQuoteLowerName(t) => impl_!(t, SingleQuoteLowerName),
            Token::RustStringLiteral(t) => impl_!(t, RustStringLiteral),
        }
    }

    pub(crate) fn into_lexer_token<'a>(self, file: FileSource<'a>) -> Token<'a> {
        use token::tagged_token::generic_tagged_token::TaggedToken;

        macro_rules! impl_ {
            ($s:ident, $Variant:ident) => {{
                let r = $s.start.byte_index..$s.end.byte_index;
                Token::$Variant(TaggedToken {
                    raw: file.get(r.into()).unwrap(),
                    _ntt: Ntt::new(),
                })
            }};
        }

        match self {
            KikiToken::StartKw(s) => impl_!(s, StartKw),
            KikiToken::StructKw(s) => impl_!(s, StructKw),
            KikiToken::EnumKw(s) => impl_!(s, EnumKw),
            KikiToken::ManualKw(s) => impl_!(s, ManualKw),
            KikiToken::TerminalKw(s) => impl_!(s, TerminalKw),
            KikiToken::BridgeKw(s) => impl_!(s, BridgeKw),
            KikiToken::UpperSelfKw(s) => impl_!(s, UpperSelfKw),
            KikiToken::Underscore(s) => impl_!(s, Underscore),
            KikiToken::LParen(s) => impl_!(s, LParen),
            KikiToken::RParen(s) => impl_!(s, RParen),
            KikiToken::LSquare(s) => impl_!(s, LSquare),
            KikiToken::RSquare(s) => impl_!(s, RSquare),
            KikiToken::LCurly(s) => impl_!(s, LCurly),
            KikiToken::RCurly(s) => impl_!(s, RCurly),
            KikiToken::Comma(s) => impl_!(s, Comma),
            KikiToken::DoubleColon(s) => impl_!(s, DoubleColon),
            KikiToken::Colon(s) => impl_!(s, Colon),
            KikiToken::Equal(s) => impl_!(s, Equal),
            KikiToken::Octothorpe(s) => impl_!(s, Octothorpe),
            KikiToken::UpperName(s) => impl_!(s, UpperName),
            KikiToken::LowerName(s) => impl_!(s, LowerName),
            KikiToken::DollarUpperName(s) => impl_!(s, DollarUpperName),
            KikiToken::AtUpperName(s) => impl_!(s, AtUpperName),
            KikiToken::AtLowerName(s) => impl_!(s, AtLowerName),
            KikiToken::SingleQuoteLowerName(s) => impl_!(s, SingleQuoteLowerName),
            KikiToken::RustStringLiteral(s) => impl_!(s, RustStringLiteral),
        }
    }
}
