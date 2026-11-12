use crate::internal_prelude::*;

use crate::raw_parser as r;

impl From<r::File> for m::File {
    fn from(value: r::File) -> Self {
        let r::File { items } = value;
        Self {
            items: (*items).into(),
        }
    }
}

impl From<r::OptItems> for Vec<m::FileItem> {
    fn from(mut value: r::OptItems) -> Self {
        let mut out = Vec::new();
        loop {
            match value {
                r::OptItems::Zero => break,
                r::OptItems::Snoc(left, right) => {
                    out.push((*right).into());
                    value = *left;
                }
            }
        }
        out.reverse();
        out
    }
}

impl From<r::FileItem> for m::FileItem {
    fn from(value: r::FileItem) -> Self {
        match value {
            r::FileItem::Start(_, name) => Self::Start(name),
            r::FileItem::Struct(value) => Self::Struct((*value).into()),
            r::FileItem::Enum(value) => Self::Enum((*value).into()),
            r::FileItem::Manual(value) => Self::Manual((*value).into()),
            r::FileItem::Terminal(value) => Self::Terminal((*value).into()),
            r::FileItem::Bridge(_, path) => Self::Bridge((*path).into()),
        }
    }
}

impl From<r::Struct> for m::Struct {
    fn from(value: r::Struct) -> Self {
        let r::Struct {
            prefix,
            name,
            fieldset,
        } = value;
        Self {
            prefix: (*prefix).into(),
            name,
            fieldset: (*fieldset).into(),
        }
    }
}

impl From<r::Enum> for m::Enum {
    fn from(value: r::Enum) -> Self {
        let r::Enum {
            prefix,
            name,
            variants,
        } = value;
        Self {
            prefix: (*prefix).into(),
            name,
            variants: (*variants).into(),
        }
    }
}

impl From<r::Manual> for m::Manual {
    fn from(value: r::Manual) -> Self {
        let r::Manual {
            name,
            override_,
            variants,
        } = value;
        Self {
            name,
            override_: (*override_).into(),
            variants: (*variants).into(),
        }
    }
}

impl From<r::TerminalEnum> for m::TerminalEnum {
    fn from(value: r::TerminalEnum) -> Self {
        let r::TerminalEnum {
            prefix,
            name,
            variants,
        } = value;
        Self {
            prefix: (*prefix).into(),
            name,
            variants: (*variants).into(),
        }
    }
}

impl From<r::OptRustStringLiteral> for Option<span_of::RustStringLiteral> {
    fn from(value: r::OptRustStringLiteral) -> Self {
        match value {
            r::OptRustStringLiteral::None => None,
            r::OptRustStringLiteral::Some(span) => Some(span),
        }
    }
}

impl From<r::Fieldset> for m::Fieldset {
    fn from(value: r::Fieldset) -> Self {
        match value {
            r::Fieldset::Empty => Self::Empty,
            r::Fieldset::Named(value) => Self::Named((*value).into()),
            r::Fieldset::Tuple(value) => Self::Tuple((*value).into()),
        }
    }
}

impl From<r::NamedFieldset> for m::NamedFieldset {
    fn from(value: r::NamedFieldset) -> Self {
        let r::NamedFieldset { fields } = value;
        Self {
            fields: (*fields).into(),
        }
    }
}

impl From<r::NamedFields> for Vec<m::NamedField> {
    fn from(mut value: r::NamedFields) -> Self {
        let mut out = Vec::new();
        loop {
            match value {
                r::NamedFields::One(item) => {
                    out.push((*item).into());
                    break;
                }
                r::NamedFields::Snoc(left, right) => {
                    out.push((*right).into());
                    value = *left;
                }
            }
        }
        out.reverse();
        out
    }
}

impl From<r::NamedField> for m::NamedField {
    fn from(value: r::NamedField) -> Self {
        let r::NamedField { name, symbol } = value;
        Self {
            name: *name,
            symbol: *symbol,
        }
    }
}

impl From<r::TupleFieldset> for m::TupleFieldset {
    fn from(value: r::TupleFieldset) -> Self {
        let r::TupleFieldset { fields } = value;
        Self {
            fields: (*fields).into(),
        }
    }
}

impl From<r::TupleFields> for Vec<m::TupleField> {
    fn from(mut value: r::TupleFields) -> Self {
        let mut out = Vec::new();
        loop {
            match value {
                r::TupleFields::One(item) => {
                    out.push((*item).into());
                    break;
                }
                r::TupleFields::Snoc(left, right) => {
                    out.push((*right).into());
                    value = *left;
                }
            }
        }
        out.reverse();
        out
    }
}

impl From<r::TupleField> for m::TupleField {
    fn from(value: r::TupleField) -> Self {
        match value {
            r::TupleField::Used(symbol) => Self::Used(*symbol),
            r::TupleField::Skipped(_, _, symbol) => Self::Skipped(*symbol),
        }
    }
}

impl From<r::EnumVariants> for Vec<m::EnumVariant> {
    fn from(mut value: r::EnumVariants) -> Self {
        let mut out = Vec::new();
        loop {
            match value {
                r::EnumVariants::One(item) => {
                    out.push((*item).into());
                    break;
                }
                r::EnumVariants::Snoc(left, right) => {
                    out.push((*right).into());
                    value = *left;
                }
            }
        }
        out.reverse();
        out
    }
}

impl From<r::EnumVariant> for m::EnumVariant {
    fn from(value: r::EnumVariant) -> Self {
        let r::EnumVariant { name, fieldset } = value;
        Self {
            name,
            fieldset: (*fieldset).into(),
        }
    }
}

impl From<r::OptManualVariants> for Vec<m::ManualVariant> {
    fn from(mut value: r::OptManualVariants) -> Self {
        let mut out = Vec::new();
        loop {
            match value {
                r::OptManualVariants::Zero => break,
                r::OptManualVariants::Snoc(left, right) => {
                    out.push((*right).into());
                    value = *left;
                }
            }
        }
        out.reverse();
        out
    }
}

impl From<r::ManualVariant> for m::ManualVariant {
    fn from(value: r::ManualVariant) -> Self {
        let r::ManualVariant { function, inputs } = value;
        Self {
            function,
            inputs: (*inputs).into(),
        }
    }
}

impl From<r::OptTupleFields> for Vec<m::TupleField> {
    fn from(value: r::OptTupleFields) -> Self {
        match value {
            r::OptTupleFields::None => Vec::new(),
            r::OptTupleFields::Some(fields) => (*fields).into(),
        }
    }
}

impl From<r::OptTerminalEnumVariants> for Vec<m::TerminalEnumVariant> {
    fn from(mut value: r::OptTerminalEnumVariants) -> Self {
        let mut out = Vec::new();
        loop {
            match value {
                r::OptTerminalEnumVariants::Zero => break,
                r::OptTerminalEnumVariants::Snoc(left, right) => {
                    out.push((*right).into());
                    value = *left;
                }
            }
        }
        out.reverse();
        out
    }
}

impl From<r::TerminalEnumVariant> for m::TerminalEnumVariant {
    fn from(value: r::TerminalEnumVariant) -> Self {
        let r::TerminalEnumVariant { name, override_ } = value;
        Self {
            name,
            override_: (*override_).into(),
        }
    }
}

impl From<r::OptEqualAtUpperName> for Option<span_of::AtUpperName> {
    fn from(value: r::OptEqualAtUpperName) -> Self {
        match value {
            r::OptEqualAtUpperName::None => None,
            r::OptEqualAtUpperName::Some(_, name) => Some(name),
        }
    }
}

impl From<r::Path> for m::Path {
    fn from(mut value: r::Path) -> Self {
        let mut out = Vec::new();
        loop {
            match value {
                r::Path::One(name) => {
                    out.push(*name);
                    break;
                }
                r::Path::Snoc(left, _, right) => {
                    out.push(*right);
                    value = *left;
                }
            }
        }
        out.reverse();
        out
    }
}
