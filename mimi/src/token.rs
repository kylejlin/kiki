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
    CustomKw,
    TerminalKw,

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransposedToken {
    // TODO
}

impl Token {
    pub const fn transpose(self) -> TransposedToken {
        let Self { kind, span } = self;
        match kind {
            _ => todo!(),
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
            _ => todo!(),
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
    // TODO: Add the rest.

    pub mod ntt {
        pub struct StartKw;
        // TODO: Add the rest
    }
}
