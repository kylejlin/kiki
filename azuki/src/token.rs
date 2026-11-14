use crate::internal_prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Token<'a> {
    StartKw(StartKw<'a>),
    StructKw(StructKw),
    EnumKw(EnumKw),
    ManualKw(ManualKw),
    TerminalKw(TerminalKw),
    BridgeKw(BridgeKw),
    Underscore(Underscore),
    LParen(LParen),
    RParen(RParen),
    LSquare(LSquare),
    RSquare(RSquare),
    LCurly(LCurly),
    RCurly(RCurly),
    Comma(Comma),
    DoubleColon(DoubleColon),
    Colon(Colon),
    Equal(Equal),
    Octothorpe(Octothorpe),
    UpperName(UpperName),
    LowerName(LowerName),
    DollarUpperName(DollarUpperName),
    AtUpperName(AtUpperName),
    AtLowerName(AtLowerName),
    RustStringLiteral(RustStringLiteral),
}

pub use tagged_token::variants::*;

pub mod tagged_token {
    pub mod variants {
        use super::{generic_tagged_token::TaggedToken, ntt};

        pub type StartKw<'a> = TaggedToken<'a, ntt::StartKw>;
        pub type StructKw = TaggedToken<ntt::StructKw>;
        pub type EnumKw = TaggedToken<ntt::EnumKw>;
        pub type ManualKw = TaggedToken<ntt::ManualKw>;
        pub type TerminalKw = TaggedToken<ntt::TerminalKw>;
        pub type BridgeKw = TaggedToken<ntt::BridgeKw>;

        pub type Underscore = TaggedToken<ntt::Underscore>;
        pub type LParen = TaggedToken<ntt::LParen>;
        pub type RParen = TaggedToken<ntt::RParen>;
        pub type LSquare = TaggedToken<ntt::LSquare>;
        pub type RSquare = TaggedToken<ntt::RSquare>;
        pub type LCurly = TaggedToken<ntt::LCurly>;
        pub type RCurly = TaggedToken<ntt::RCurly>;
        pub type Comma = TaggedToken<ntt::Comma>;
        pub type DoubleColon = TaggedToken<ntt::DoubleColon>;
        pub type Colon = TaggedToken<ntt::Colon>;
        pub type Equal = TaggedToken<ntt::Equal>;
        pub type Octothorpe = TaggedToken<ntt::Octothorpe>;

        pub type UpperName = TaggedToken<ntt::UpperName>;
        pub type LowerName = TaggedToken<ntt::LowerName>;
        pub type DollarUpperName = TaggedToken<ntt::DollarUpperName>;
        pub type AtUpperName = TaggedToken<ntt::AtUpperName>;
        pub type AtLowerName = TaggedToken<ntt::AtLowerName>;

        pub type RustStringLiteral = TaggedToken<ntt::RustStringLiteral>;
    }

    pub mod generic_tagged_token {
        use crate::{
            internal_prelude::{Debug, Formatter, Hash, Hasher, Ntt, Span, fmt},
            source_file::SourceFragment,
        };

        pub struct TaggedToken<'a, T> {
            pub raw: SourceFragment<'a>,
            pub _ntt: Ntt<T>,
        }
        impl<T> Debug for TaggedToken<T> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                self.raw.fmt(f)
            }
        }
        impl<T> Clone for TaggedToken<T> {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl<T> Copy for TaggedToken<T> {}
        impl<T> PartialEq for TaggedToken<T> {
            fn eq(&self, other: &Self) -> bool {
                self.raw == other.raw
            }
        }
        impl<T> Eq for TaggedToken<T> {}
        impl<T> Hash for TaggedToken<T> {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.raw.hash(state)
            }
        }

        impl<T> TaggedToken<T> {
            pub const fn new(raw: Span) -> Self {
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
