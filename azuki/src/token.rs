use crate::internal_prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    StartKw,
    StructKw,
    EnumKw,
    ManualKw,
    TerminalKw,
    BridgeKw,

    Underscore,
    LParen,
    RParen,
    LSquare,
    RSquare,
    LCurly,
    RCurly,
    Comma,
    DoubleColon,
    Colon,
    Equal,
    Octothorpe,

    UpperName,
    LowerName,
    DollarUpperName,
    AtUpperName,
    AtLowerName,

    RustStringLiteral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransposedToken {
    StartKw(crate::token::span_of::StartKw),
    StructKw(crate::token::span_of::StructKw),
    EnumKw(crate::token::span_of::EnumKw),
    ManualKw(crate::token::span_of::ManualKw),
    TerminalKw(crate::token::span_of::TerminalKw),
    BridgeKw(crate::token::span_of::BridgeKw),
    Underscore(crate::token::span_of::Underscore),
    LParen(crate::token::span_of::LParen),
    RParen(crate::token::span_of::RParen),
    LSquare(crate::token::span_of::LSquare),
    RSquare(crate::token::span_of::RSquare),
    LCurly(crate::token::span_of::LCurly),
    RCurly(crate::token::span_of::RCurly),
    Comma(crate::token::span_of::Comma),
    DoubleColon(crate::token::span_of::DoubleColon),
    Colon(crate::token::span_of::Colon),
    Equal(crate::token::span_of::Equal),
    Octothorpe(crate::token::span_of::Octothorpe),
    UpperName(crate::token::span_of::UpperName),
    LowerName(crate::token::span_of::LowerName),
    DollarUpperName(crate::token::span_of::DollarUpperName),
    AtUpperName(crate::token::span_of::AtUpperName),
    AtLowerName(crate::token::span_of::AtLowerName),
    RustStringLiteral(crate::token::span_of::RustStringLiteral),
}

impl Token {
    pub const fn transpose(self) -> TransposedToken {
        let Self { kind, span } = self;
        match kind {
            TokenKind::StartKw => TransposedToken::StartKw(span_of::StartKw::new(span)),
            TokenKind::StructKw => TransposedToken::StructKw(span_of::StructKw::new(span)),
            TokenKind::EnumKw => TransposedToken::EnumKw(span_of::EnumKw::new(span)),
            TokenKind::ManualKw => TransposedToken::ManualKw(span_of::ManualKw::new(span)),
            TokenKind::TerminalKw => TransposedToken::TerminalKw(span_of::TerminalKw::new(span)),
            TokenKind::BridgeKw => TransposedToken::BridgeKw(span_of::BridgeKw::new(span)),
            TokenKind::Underscore => TransposedToken::Underscore(span_of::Underscore::new(span)),
            TokenKind::LParen => TransposedToken::LParen(span_of::LParen::new(span)),
            TokenKind::RParen => TransposedToken::RParen(span_of::RParen::new(span)),
            TokenKind::LSquare => TransposedToken::LSquare(span_of::LSquare::new(span)),
            TokenKind::RSquare => TransposedToken::RSquare(span_of::RSquare::new(span)),
            TokenKind::LCurly => TransposedToken::LCurly(span_of::LCurly::new(span)),
            TokenKind::RCurly => TransposedToken::RCurly(span_of::RCurly::new(span)),
            TokenKind::Comma => TransposedToken::Comma(span_of::Comma::new(span)),
            TokenKind::DoubleColon => TransposedToken::DoubleColon(span_of::DoubleColon::new(span)),
            TokenKind::Colon => TransposedToken::Colon(span_of::Colon::new(span)),
            TokenKind::Equal => TransposedToken::Equal(span_of::Equal::new(span)),
            TokenKind::Octothorpe => TransposedToken::Octothorpe(span_of::Octothorpe::new(span)),
            TokenKind::UpperName => TransposedToken::UpperName(span_of::UpperName::new(span)),
            TokenKind::LowerName => TransposedToken::LowerName(span_of::LowerName::new(span)),
            TokenKind::DollarUpperName => {
                TransposedToken::DollarUpperName(span_of::DollarUpperName::new(span))
            }
            TokenKind::AtUpperName => TransposedToken::AtUpperName(span_of::AtUpperName::new(span)),
            TokenKind::AtLowerName => TransposedToken::AtLowerName(span_of::AtLowerName::new(span)),
            TokenKind::RustStringLiteral => {
                TransposedToken::RustStringLiteral(span_of::RustStringLiteral::new(span))
            }
        }
    }
}
impl From<Token> for TransposedToken {
    fn from(token: Token) -> Self {
        token.transpose()
    }
}

impl TransposedToken {
    pub const fn transpose(self) -> Token {
        match self {
            Self::StartKw(span) => Token {
                kind: TokenKind::StartKw,
                span: span.raw,
            },
            Self::StructKw(span) => Token {
                kind: TokenKind::StructKw,
                span: span.raw,
            },
            Self::EnumKw(span) => Token {
                kind: TokenKind::EnumKw,
                span: span.raw,
            },
            Self::ManualKw(span) => Token {
                kind: TokenKind::ManualKw,
                span: span.raw,
            },
            Self::TerminalKw(span) => Token {
                kind: TokenKind::TerminalKw,
                span: span.raw,
            },
            Self::BridgeKw(span) => Token {
                kind: TokenKind::BridgeKw,
                span: span.raw,
            },
            Self::Underscore(span) => Token {
                kind: TokenKind::Underscore,
                span: span.raw,
            },
            Self::LParen(span) => Token {
                kind: TokenKind::LParen,
                span: span.raw,
            },
            Self::RParen(span) => Token {
                kind: TokenKind::RParen,
                span: span.raw,
            },
            Self::LSquare(span) => Token {
                kind: TokenKind::LSquare,
                span: span.raw,
            },
            Self::RSquare(span) => Token {
                kind: TokenKind::RSquare,
                span: span.raw,
            },
            Self::LCurly(span) => Token {
                kind: TokenKind::LCurly,
                span: span.raw,
            },
            Self::RCurly(span) => Token {
                kind: TokenKind::RCurly,
                span: span.raw,
            },
            Self::Comma(span) => Token {
                kind: TokenKind::Comma,
                span: span.raw,
            },
            Self::DoubleColon(span) => Token {
                kind: TokenKind::DoubleColon,
                span: span.raw,
            },
            Self::Colon(span) => Token {
                kind: TokenKind::Colon,
                span: span.raw,
            },
            Self::Equal(span) => Token {
                kind: TokenKind::Equal,
                span: span.raw,
            },
            Self::Octothorpe(span) => Token {
                kind: TokenKind::Octothorpe,
                span: span.raw,
            },
            Self::UpperName(span) => Token {
                kind: TokenKind::UpperName,
                span: span.raw,
            },
            Self::LowerName(span) => Token {
                kind: TokenKind::LowerName,
                span: span.raw,
            },
            Self::DollarUpperName(span) => Token {
                kind: TokenKind::DollarUpperName,
                span: span.raw,
            },
            Self::AtUpperName(span) => Token {
                kind: TokenKind::AtUpperName,
                span: span.raw,
            },
            Self::AtLowerName(span) => Token {
                kind: TokenKind::AtLowerName,
                span: span.raw,
            },
            Self::RustStringLiteral(span) => Token {
                kind: TokenKind::RustStringLiteral,
                span: span.raw,
            },
        }
    }
}
impl From<TransposedToken> for Token {
    fn from(token: TransposedToken) -> Self {
        token.transpose()
    }
}

pub mod span_of {
    use crate::internal_prelude::{Debug, Formatter, Hash, Hasher, Ntt, Span, fmt};

    pub struct TaggedSpan<T> {
        pub raw: Span,
        pub _ntt: Ntt<T>,
    }
    impl<T> Debug for TaggedSpan<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            self.raw.fmt(f)
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
            self.raw == other.raw
        }
    }
    impl<T> Eq for TaggedSpan<T> {}
    impl<T> Hash for TaggedSpan<T> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.raw.hash(state)
        }
    }

    impl<T> TaggedSpan<T> {
        pub const fn new(raw: Span) -> Self {
            Self {
                raw,
                _ntt: Ntt::new(),
            }
        }
    }

    pub type StartKw = TaggedSpan<ntt::StartKw>;
    pub type StructKw = TaggedSpan<ntt::StructKw>;
    pub type EnumKw = TaggedSpan<ntt::EnumKw>;
    pub type ManualKw = TaggedSpan<ntt::ManualKw>;
    pub type TerminalKw = TaggedSpan<ntt::TerminalKw>;
    pub type BridgeKw = TaggedSpan<ntt::BridgeKw>;

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

    pub type RustStringLiteral = TaggedSpan<ntt::RustStringLiteral>;

    pub mod ntt {
        pub struct StartKw;
        pub struct StructKw;
        pub struct EnumKw;
        pub struct ManualKw;
        pub struct TerminalKw;
        pub struct BridgeKw;

        pub struct Underscore;
        pub struct LParen;
        pub struct RParen;
        pub struct LSquare;
        pub struct RSquare;
        pub struct LCurly;
        pub struct RCurly;
        pub struct Comma;
        pub struct DoubleColon;
        pub struct Colon;
        pub struct Equal;
        pub struct Octothorpe;

        pub struct UpperName;
        pub struct LowerName;
        pub struct DollarUpperName;
        pub struct AtUpperName;
        pub struct AtLowerName;

        pub struct RustStringLiteral;
    }
}
