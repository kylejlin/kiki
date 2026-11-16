use super::*;

use raw_parser as r;

pub(super) trait Massage<'a> {
    type Out;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out;
}

impl<'a> Massage<'a> for r::File {
    type Out = File<'a>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        let r::File { items } = self;
        Self::Out {
            items: items.massage(file_source),
        }
    }
}

impl<'a> Massage<'a> for r::OptItems {
    type Out = Vec<FileItem<'a>>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        let mut value = self;
        let mut out = Vec::new();
        loop {
            match value {
                r::OptItems::Zero => break,
                r::OptItems::Snoc(left, right) => {
                    out.push(right.massage(file_source));
                    value = left;
                }
            }
        }
        out.reverse();
        out
    }
}

impl<'a> Massage<'a> for r::FileItem {
    type Out = FileItem<'a>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        match self {
            r::FileItem::Start(_, name) => Self::Out::Start(name.massage(file_source)),
            r::FileItem::Struct(value) => Self::Out::Struct(value.massage(file_source)),
            r::FileItem::Enum(value) => Self::Out::Enum(value.massage(file_source)),
            r::FileItem::ManualNonTerminal(value) => {
                Self::Out::ManualNonTerminal(value.massage(file_source))
            }
            r::FileItem::GeneratedTerminal(value) => {
                Self::Out::GeneratedTerminal(value.massage(file_source))
            }
            r::FileItem::ManualTerminal(value) => {
                Self::Out::ManualTerminal(value.massage(file_source))
            }
            r::FileItem::Bridge(_, path) => Self::Out::Bridge(path.massage(file_source)),
        }
    }
}

impl<'a> Massage<'a> for r::Struct {
    type Out = Struct<'a>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        let r::Struct {
            prefix,
            name,
            fieldset,
        } = self;
        Self::Out {
            prefix: prefix.massage(file_source),
            name: name.massage(file_source),
            fieldset: fieldset.massage(file_source),
        }
    }
}

impl<'a> Massage<'a> for r::Enum {
    type Out = Enum<'a>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        let r::Enum {
            prefix,
            name,
            variants,
        } = self;
        Self::Out {
            prefix: prefix.massage(file_source),
            name: name.massage(file_source),
            variants: variants.massage(file_source),
        }
    }
}

impl<'a> Massage<'a> for r::ManualNonTerminal {
    type Out = ManualNonTerminal<'a>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        let r::ManualNonTerminal {
            name,
            override_,
            lifetimes,
            variants,
        } = self;
        Self::Out {
            name: name.massage(file_source),
            override_: override_.massage(file_source),
            lifetimes: lifetimes.massage(file_source),
            variants: variants.massage(file_source),
        }
    }
}

impl<'a> Massage<'a> for r::GeneratedTerminal {
    type Out = GeneratedTerminal<'a>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        let r::GeneratedTerminal {
            prefix,
            name,
            global_lifetimes,
            variants,
        } = self;
        Self::Out {
            prefix: prefix.massage(file_source),
            name: name.massage(file_source),
            global_lifetimes: global_lifetimes.massage(file_source),
            variants: variants.massage(file_source),
        }
    }
}

impl<'a> Massage<'a> for r::ManualTerminal {
    type Out = ManualTerminal<'a>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        let r::ManualTerminal {
            name,
            override_,
            global_lifetimes,
            variants,
        } = self;
        Self::Out {
            name: name.massage(file_source),
            override_: override_.massage(file_source),
            global_lifetimes: global_lifetimes.massage(file_source),
            variants: variants.massage(file_source),
        }
    }
}

impl<'a> Massage<'a> for r::OptRustStringLiteral {
    type Out = Option<token::RustStringLiteral<'a>>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        match self {
            r::OptRustStringLiteral::None => None,
            r::OptRustStringLiteral::Some(span) => Some(span.massage(file_source)),
        }
    }
}

impl<'a> Massage<'a> for r::Fieldset {
    type Out = Fieldset<'a>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        match self {
            r::Fieldset::Empty => Self::Out::Empty,
            r::Fieldset::Named(value) => Self::Out::Named(value.massage(file_source)),
            r::Fieldset::Tuple(value) => Self::Out::Tuple(value.massage(file_source)),
        }
    }
}

impl<'a> Massage<'a> for r::NamedFieldset {
    type Out = NamedFieldset<'a>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        let r::NamedFieldset { fields } = self;
        Self::Out {
            fields: fields.massage(file_source),
        }
    }
}

impl<'a> Massage<'a> for r::NamedFields {
    type Out = Vec<NamedField<'a>>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        let mut value = self;
        let mut out = Vec::new();
        loop {
            match value {
                r::NamedFields::One(item) => {
                    out.push(item.massage(file_source));
                    break;
                }
                r::NamedFields::Snoc(left, right) => {
                    out.push(right.massage(file_source));
                    value = left;
                }
            }
        }
        out.reverse();
        out
    }
}

impl<'a> Massage<'a> for r::NamedField {
    type Out = NamedField<'a>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        let r::NamedField { name, symbol } = self;
        Self::Out {
            name: name.massage(file_source),
            symbol: symbol.massage(file_source),
        }
    }
}

impl<'a> Massage<'a> for r::TupleFieldset {
    type Out = TupleFieldset<'a>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        let r::TupleFieldset { fields } = self;
        Self::Out {
            fields: fields.massage(file_source),
        }
    }
}

impl<'a> Massage<'a> for r::TupleFields {
    type Out = Vec<TupleField<'a>>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        let mut value = self;
        let mut out = Vec::new();
        loop {
            match value {
                r::TupleFields::One(item) => {
                    out.push(item.massage(file_source));
                    break;
                }
                r::TupleFields::Snoc(left, right) => {
                    out.push(right.massage(file_source));
                    value = left;
                }
            }
        }
        out.reverse();
        out
    }
}

impl<'a> Massage<'a> for r::TupleField {
    type Out = TupleField<'a>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        match self {
            r::TupleField::Used(symbol) => Self::Out::Used(symbol.massage(file_source)),
            r::TupleField::Skipped(_, _, symbol) => Self::Out::Skipped(symbol.massage(file_source)),
        }
    }
}

impl<'a> Massage<'a> for r::EnumVariants {
    type Out = Vec<EnumVariant<'a>>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        let mut value = self;
        let mut out = Vec::new();
        loop {
            match value {
                r::EnumVariants::One(item) => {
                    out.push(item.massage(file_source));
                    break;
                }
                r::EnumVariants::Snoc(left, right) => {
                    out.push(right.massage(file_source));
                    value = left;
                }
            }
        }
        out.reverse();
        out
    }
}

impl<'a> Massage<'a> for r::EnumVariant {
    type Out = EnumVariant<'a>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        let r::EnumVariant { name, fieldset } = self;
        Self::Out {
            name: name.massage(file_source),
            fieldset: fieldset.massage(file_source),
        }
    }
}

impl<'a> Massage<'a> for r::ManualNonTerminalVariants {
    type Out = Vec<ManualNonTerminalVariant<'a>>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        let mut value = self;
        let mut out = Vec::new();
        loop {
            match value {
                r::ManualNonTerminalVariants::One(item) => {
                    out.push(item.massage(file_source));
                    break;
                }
                r::ManualNonTerminalVariants::Snoc(left, right) => {
                    out.push(right.massage(file_source));
                    value = left;
                }
            }
        }
        out.reverse();
        out
    }
}

impl<'a> Massage<'a> for r::ManualNonTerminalVariant {
    type Out = ManualNonTerminalVariant<'a>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        let r::ManualNonTerminalVariant { function, inputs } = self;
        Self::Out {
            function: function.massage(file_source),
            inputs: inputs.massage(file_source),
        }
    }
}

impl<'a> Massage<'a> for r::OptTupleFieldset {
    type Out = Vec<TupleField<'a>>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        match self {
            r::OptTupleFieldset::None => Vec::new(),
            r::OptTupleFieldset::Some(fieldset) => {
                let r::TupleFieldset { fields } = &**fieldset;
                fields.massage(file_source)
            }
        }
    }
}

impl<'a> Massage<'a> for r::OptTerminalEnumVariants {
    type Out = Vec<TerminalEnumVariant<'a>>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        let mut value = self;
        let mut out = Vec::new();
        loop {
            match value {
                r::OptTerminalEnumVariants::Zero => break,
                r::OptTerminalEnumVariants::Snoc(left, right) => {
                    out.push(right.massage(file_source));
                    value = left;
                }
            }
        }
        out.reverse();
        out
    }
}

impl<'a> Massage<'a> for r::TerminalEnumVariant {
    type Out = TerminalEnumVariant<'a>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        let r::TerminalEnumVariant {
            name,
            override_,
            lifetimes,
        } = self;
        Self::Out {
            name: name.massage(file_source),
            override_: override_.massage(file_source),
            lifetimes: lifetimes.massage(file_source),
        }
    }
}

impl<'a> Massage<'a> for r::OptEqualAtUpperName {
    type Out = Option<token::AtUpperName<'a>>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        match self {
            r::OptEqualAtUpperName::None => None,
            r::OptEqualAtUpperName::Some(_, name) => Some(name.massage(file_source)),
        }
    }
}

impl<'a> Massage<'a> for r::Path {
    type Out = Path<'a>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        let mut value = self;
        let mut out = Vec::new();
        loop {
            match value {
                r::Path::One(name) => {
                    out.push(name.massage(file_source));
                    break;
                }
                r::Path::Snoc(left, _, right) => {
                    out.push(right.massage(file_source));
                    value = left;
                }
            }
        }
        out.reverse();
        out
    }
}

impl<'a> Massage<'a> for r::OptSingleQuoteLowerNames {
    type Out = Vec<token::SingleQuoteLowerName<'a>>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        let mut value = self;
        let mut out = Vec::new();
        loop {
            match value {
                r::OptSingleQuoteLowerNames::Zero => break,
                r::OptSingleQuoteLowerNames::Snoc(left, right) => {
                    out.push(right.massage(file_source));
                    value = left;
                }
            }
        }
        out.reverse();
        out
    }
}

impl<'a> Massage<'a> for r::Name {
    type Out = Name<'a>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        match self {
            r::Name::Upper(token) => Self::Out::Upper(token.massage(file_source)),
            r::Name::Lower(token) => Self::Out::Lower(token.massage(file_source)),
        }
    }
}

impl<'a> Massage<'a> for r::UnderscoreOrLowerName {
    type Out = UnderscoreOrLowerName<'a>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        match self {
            r::UnderscoreOrLowerName::Underscore(token) => {
                Self::Out::Underscore(token.massage(file_source))
            }
            r::UnderscoreOrLowerName::Lower(token) => Self::Out::Lower(token.massage(file_source)),
        }
    }
}

impl<'a> Massage<'a> for r::GrammarSymbolOrSelf {
    type Out = GrammarSymbolOrSelf<'a>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        match self {
            r::GrammarSymbolOrSelf::Upper(token) => Self::Out::Upper(token.massage(file_source)),
            r::GrammarSymbolOrSelf::DollarUpper(token) => {
                Self::Out::DollarUpper(token.massage(file_source))
            }
            r::GrammarSymbolOrSelf::Self_(token) => Self::Out::Self_(token.massage(file_source)),
        }
    }
}

impl<'a, T> Massage<'a> for kiki_bridge::TaggedSpan<T> {
    type Out = token::tagged_token::generic_tagged_token::TaggedToken<'a, T>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        let Self { start, end, _ntt } = self;
        let range = start.byte_index..end.byte_index;
        Self::Out::new(file_source.get(range.into()).unwrap())
    }
}
