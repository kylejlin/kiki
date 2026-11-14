use crate::internal_prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Token<'a> {
    StartKw(StartKw<'a>),
    StructKw(StructKw<'a>),
    EnumKw(EnumKw<'a>),
    ManualKw(ManualKw<'a>),
    TerminalKw(TerminalKw<'a>),
    BridgeKw(BridgeKw<'a>),
    Underscore(Underscore<'a>),
    LParen(LParen<'a>),
    RParen(RParen<'a>),
    LSquare(LSquare<'a>),
    RSquare(RSquare<'a>),
    LCurly(LCurly<'a>),
    RCurly(RCurly<'a>),
    Comma(Comma<'a>),
    DoubleColon(DoubleColon<'a>),
    Colon(Colon<'a>),
    Equal(Equal<'a>),
    Octothorpe(Octothorpe<'a>),
    UpperName(UpperName<'a>),
    LowerName(LowerName<'a>),
    DollarUpperName(DollarUpperName<'a>),
    AtUpperName(AtUpperName<'a>),
    AtLowerName(AtLowerName<'a>),
    RustStringLiteral(RustStringLiteral<'a>),
}

pub use tagged_token::variants::*;

pub mod tagged_token {
    pub mod variants {
        use super::{generic_tagged_token::TaggedToken, ntt};

        pub type StartKw<'a> = TaggedToken<'a, ntt::StartKw>;
        pub type StructKw<'a> = TaggedToken<'a, ntt::StructKw>;
        pub type EnumKw<'a> = TaggedToken<'a, ntt::EnumKw>;
        pub type ManualKw<'a> = TaggedToken<'a, ntt::ManualKw>;
        pub type TerminalKw<'a> = TaggedToken<'a, ntt::TerminalKw>;
        pub type BridgeKw<'a> = TaggedToken<'a, ntt::BridgeKw>;

        pub type Underscore<'a> = TaggedToken<'a, ntt::Underscore>;
        pub type LParen<'a> = TaggedToken<'a, ntt::LParen>;
        pub type RParen<'a> = TaggedToken<'a, ntt::RParen>;
        pub type LSquare<'a> = TaggedToken<'a, ntt::LSquare>;
        pub type RSquare<'a> = TaggedToken<'a, ntt::RSquare>;
        pub type LCurly<'a> = TaggedToken<'a, ntt::LCurly>;
        pub type RCurly<'a> = TaggedToken<'a, ntt::RCurly>;
        pub type Comma<'a> = TaggedToken<'a, ntt::Comma>;
        pub type DoubleColon<'a> = TaggedToken<'a, ntt::DoubleColon>;
        pub type Colon<'a> = TaggedToken<'a, ntt::Colon>;
        pub type Equal<'a> = TaggedToken<'a, ntt::Equal>;
        pub type Octothorpe<'a> = TaggedToken<'a, ntt::Octothorpe>;

        pub type UpperName<'a> = TaggedToken<'a, ntt::UpperName>;
        pub type LowerName<'a> = TaggedToken<'a, ntt::LowerName>;
        pub type DollarUpperName<'a> = TaggedToken<'a, ntt::DollarUpperName>;
        pub type AtUpperName<'a> = TaggedToken<'a, ntt::AtUpperName>;
        pub type AtLowerName<'a> = TaggedToken<'a, ntt::AtLowerName>;

        pub type RustStringLiteral<'a> = TaggedToken<'a, ntt::RustStringLiteral>;
    }

    pub mod generic_tagged_token {
        use crate::{
            internal_prelude::{Debug, Formatter, Hash, Hasher, Ntt, fmt},
            source_file::SourceFragment,
        };

        pub struct TaggedToken<'a, T> {
            pub raw: SourceFragment<'a>,
            pub _ntt: Ntt<T>,
        }
        impl<T> Debug for TaggedToken<'_, T> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                self.raw.fmt(f)
            }
        }
        impl<T> Clone for TaggedToken<'_, T> {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl<T> Copy for TaggedToken<'_, T> {}
        impl<T> PartialEq for TaggedToken<'_, T> {
            fn eq(&self, other: &Self) -> bool {
                self.raw == other.raw
            }
        }
        impl<T> Eq for TaggedToken<'_, T> {}
        impl<T> Hash for TaggedToken<'_, T> {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.raw.hash(state)
            }
        }

        impl<'a, T> TaggedToken<'a, T> {
            pub const fn new(raw: SourceFragment<'a>) -> Self {
                Self {
                    raw,
                    _ntt: Ntt::new(),
                }
            }
        }
    }

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
