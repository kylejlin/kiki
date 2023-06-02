use crate::data::{ast, cst};

impl From<cst::File> for ast::File {
    fn from(cst: cst::File) -> Self {
        ast::File {
            items: cst.items.into(),
        }
    }
}

impl From<cst::OptItems> for Vec<ast::FileItem> {
    fn from(cst: cst::OptItems) -> Self {
        match cst {
            cst::OptItems::Nil => vec![],
            cst::OptItems::Cons(left, right) => {
                let mut items: Vec<ast::FileItem> = (*left).into();
                items.push(right.into());
                items
            }
        }
    }
}

impl From<cst::FileItem> for ast::FileItem {
    fn from(cst: cst::FileItem) -> Self {
        match cst {
            cst::FileItem::Start(ident) => ast::FileItem::Start(ident.into()),
            cst::FileItem::Struct(struct_) => ast::FileItem::Struct(struct_.into()),
            cst::FileItem::Enum(enum_) => ast::FileItem::Enum(enum_.into()),
            cst::FileItem::Terminal(terminal) => ast::FileItem::Terminal(terminal.into()),
        }
    }
}

impl From<cst::Struct> for ast::Struct {
    fn from(cst: cst::Struct) -> Self {
        ast::Struct {
            name: cst.name.into(),
            fieldset: cst.fieldset.into(),
        }
    }
}

impl From<cst::Enum> for ast::Enum {
    fn from(cst: cst::Enum) -> Self {
        ast::Enum {
            name: cst.name.into(),
            variants: cst.variants.into(),
        }
    }
}

impl From<cst::TerminalEnum> for ast::TerminalEnum {
    fn from(cst: cst::TerminalEnum) -> Self {
        ast::TerminalEnum {
            name: cst.name.into(),
            variants: cst.variants.into(),
        }
    }
}

impl From<cst::Fieldset> for ast::Fieldset {
    fn from(cst: cst::Fieldset) -> Self {
        match cst {
            cst::Fieldset::Empty => ast::Fieldset::Empty,
            cst::Fieldset::Named(named_fieldset) => ast::Fieldset::Named(named_fieldset.into()),
            cst::Fieldset::Tuple(tuple_fieldset) => ast::Fieldset::Tuple(tuple_fieldset.into()),
        }
    }
}

impl From<cst::NamedFieldset> for ast::NamedFieldset {
    fn from(cst: cst::NamedFieldset) -> Self {
        ast::NamedFieldset {
            fields: cst.fields.into(),
        }
    }
}

impl From<cst::NamedFields> for Vec<ast::NamedField> {
    fn from(cst: cst::NamedFields) -> Self {
        match cst {
            cst::NamedFields::One(named_field) => vec![named_field.into()],
            cst::NamedFields::Cons(left, right) => {
                let mut fields: Vec<ast::NamedField> = (*left).into();
                fields.push(right.into());
                fields
            }
        }
    }
}

impl From<cst::NamedField> for ast::NamedField {
    fn from(cst: cst::NamedField) -> Self {
        ast::NamedField {
            name: cst.name.into(),
            symbol: cst.symbol.into(),
        }
    }
}

impl From<cst::TupleFieldset> for ast::TupleFieldset {
    fn from(cst: cst::TupleFieldset) -> Self {
        ast::TupleFieldset {
            fields: cst.fields.into(),
        }
    }
}

impl From<cst::TupleFields> for Vec<ast::TupleField> {
    fn from(cst: cst::TupleFields) -> Self {
        match cst {
            cst::TupleFields::One(field) => vec![field.into()],
            cst::TupleFields::Cons(left, right) => {
                let mut fields: Vec<ast::TupleField> = (*left).into();
                fields.push(right.into());
                fields
            }
        }
    }
}

impl From<cst::TupleField> for ast::TupleField {
    fn from(cst: cst::TupleField) -> Self {
        match cst {
            cst::TupleField::Skipped(named_field) => ast::TupleField::Skipped(named_field.into()),
            cst::TupleField::Used(symbol) => ast::TupleField::Used(symbol.into()),
        }
    }
}

impl From<cst::OptEnumVariants> for Vec<ast::EnumVariant> {
    fn from(cst: cst::OptEnumVariants) -> Self {
        match cst {
            cst::OptEnumVariants::Nil => vec![],
            cst::OptEnumVariants::Cons(left, right) => {
                let mut variants: Vec<ast::EnumVariant> = (*left).into();
                variants.push(right.into());
                variants
            }
        }
    }
}

impl From<cst::EnumVariant> for ast::EnumVariant {
    fn from(cst: cst::EnumVariant) -> Self {
        ast::EnumVariant {
            name: cst.name.into(),
            fieldset: cst.fieldset.into(),
        }
    }
}

impl From<cst::OptTerminalEnumVariants> for Vec<ast::TerminalEnumVariant> {
    fn from(cst: cst::OptTerminalEnumVariants) -> Self {
        match cst {
            cst::OptTerminalEnumVariants::Nil => vec![],
            cst::OptTerminalEnumVariants::Cons(left, right) => {
                let mut variants: Vec<ast::TerminalEnumVariant> = (*left).into();
                variants.push(right.into());
                variants
            }
        }
    }
}

impl From<cst::TerminalEnumVariant> for ast::TerminalEnumVariant {
    fn from(cst: cst::TerminalEnumVariant) -> Self {
        ast::TerminalEnumVariant {
            name: cst.name.into(),
            type_: cst.type_.into(),
        }
    }
}

impl From<cst::Type> for ast::Type {
    fn from(cst: cst::Type) -> Self {
        match cst {
            cst::Type::Unit => ast::Type::Unit,
            cst::Type::Path(path) => ast::Type::Path(path.into()),
            cst::Type::Complex(complex_type) => {
                ast::Type::Complex(Box::new((*complex_type).into()))
            }
        }
    }
}

impl From<cst::Path> for Vec<ast::Ident> {
    fn from(cst: cst::Path) -> Self {
        match cst {
            cst::Path::One(ident) => vec![ident.into()],
            cst::Path::Cons(left, right) => {
                let mut left: Vec<ast::Ident> = (*left).into();
                left.push(right.into());
                left
            }
        }
    }
}

impl From<cst::ComplexType> for ast::ComplexType {
    fn from(cst: cst::ComplexType) -> Self {
        ast::ComplexType {
            callee: cst.callee.into(),
            args: cst.args.into(),
        }
    }
}

impl From<cst::CommaSeparatedTypes> for Vec<ast::Type> {
    fn from(cst: cst::CommaSeparatedTypes) -> Self {
        match cst {
            cst::CommaSeparatedTypes::One(type_) => vec![type_.into()],
            cst::CommaSeparatedTypes::Cons(left, right) => {
                let mut types: Vec<ast::Type> = (*left).into();
                types.push(right.into());
                types
            }
        }
    }
}
