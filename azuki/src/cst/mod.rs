use crate::internal_prelude::*;

mod kiki_bridge;
mod massage;
include!(concat!(env!("OUT_DIR"), "/cst/raw_parser.rs"));

#[derive(Debug, Clone)]
pub struct ParseError<'a> {
    /// This is `None` if and only if there was an unexpected end of input.
    pub unexpected_token: Option<Token<'a>>,
}

/// If there is a lexer error, this returns `Err(Err(LexError))`.
/// If there are no lexer errors, but there is a parser error, this returns `Err(Ok(ParseError))`.
/// Otherwise, this returns `Ok`.
pub fn parse<'a>(
    file_source: FileSource<'a>,
) -> Result<File<'a>, Result<ParseError<'a>, LexError>> {
    use massage::Massage;

    let mut lexer = lex(file_source.raw());
    let mut lex_error = None;

    let tokens = lexer
        .by_ref()
        .map_while(|result| match result {
            Ok(token) => Some(token),
            Err(error) => {
                lex_error = Some(error);
                None
            }
        })
        .fuse()
        .map(|token| raw_parser::Token::from_lexer_token(token, file_source));

    let parsed = raw_parser::parse(tokens);

    if let Some(error) = lex_error.or_else(|| lexer.find_map(Result::err)) {
        return Err(Err(error));
    }

    parsed
        .map(|file| file.massage(file_source))
        .map_err(|unexpected| {
            Ok(ParseError {
                unexpected_token: unexpected.map(|token| token.into_lexer_token(file_source)),
            })
        })
}

#[derive(Debug)]
pub struct File<'a> {
    pub items: Vec<FileItem<'a>>,
}

#[derive(Debug)]
pub enum FileItem<'a> {
    Start(token::UpperName<'a>),
    Struct(Struct<'a>),
    Enum(Enum<'a>),
    ManualNonTerminal(ManualNonTerminal<'a>),
    Terminal(TerminalEnum<'a>),
    ManualTerminal(ManualTerminal<'a>),
    Bridge(Path<'a>),
}

#[derive(Debug)]
pub struct Struct<'a> {
    pub prefix: Option<token::RustStringLiteral<'a>>,
    pub name: token::UpperName<'a>,
    pub fieldset: Fieldset<'a>,
}

#[derive(Debug)]
pub struct Enum<'a> {
    pub prefix: Option<token::RustStringLiteral<'a>>,
    pub name: token::UpperName<'a>,
    pub variants: Vec<EnumVariant<'a>>,
}

#[derive(Debug)]
pub struct ManualNonTerminal<'a> {
    pub name: token::UpperName<'a>,
    pub override_: Option<token::AtUpperName<'a>>,
    pub lifetimes: Vec<token::SingleQuoteLowerName<'a>>,
    pub variants: Vec<ManualNonTerminalVariant<'a>>,
}

#[derive(Debug)]
pub struct TerminalEnum<'a> {
    pub prefix: Option<token::RustStringLiteral<'a>>,
    pub name: token::UpperName<'a>,
    pub global_lifetimes: Vec<token::SingleQuoteLowerName<'a>>,
    pub variants: Vec<TerminalEnumVariant<'a>>,
}

#[derive(Debug)]
pub struct ManualTerminal<'a> {
    pub name: token::UpperName<'a>,
    pub override_: Option<token::AtUpperName<'a>>,
    pub global_lifetimes: Vec<token::SingleQuoteLowerName<'a>>,
    pub variants: Vec<TerminalEnumVariant<'a>>,
}

#[derive(Debug)]
pub enum Fieldset<'a> {
    Empty,
    Named(NamedFieldset<'a>),
    Tuple(TupleFieldset<'a>),
}

#[derive(Debug)]
pub struct NamedFieldset<'a> {
    pub fields: Vec<NamedField<'a>>,
}

#[derive(Debug)]
pub struct NamedField<'a> {
    pub name: UnderscoreOrLowerName<'a>,
    pub symbol: GrammarSymbolOrSelf<'a>,
}

#[derive(Debug)]
pub struct TupleFieldset<'a> {
    pub fields: Vec<TupleField<'a>>,
}

#[derive(Debug)]
pub enum TupleField<'a> {
    Used(GrammarSymbolOrSelf<'a>),
    Skipped(GrammarSymbolOrSelf<'a>),
}

#[derive(Debug)]
pub struct EnumVariant<'a> {
    pub name: token::UpperName<'a>,
    pub fieldset: Fieldset<'a>,
}

#[derive(Debug)]
pub struct ManualNonTerminalVariant<'a> {
    pub function: token::AtLowerName<'a>,
    pub inputs: Vec<TupleField<'a>>,
}

#[derive(Debug)]
pub struct TerminalEnumVariant<'a> {
    pub name: token::DollarUpperName<'a>,
    pub override_: Option<token::AtUpperName<'a>>,
    pub lifetimes: Vec<token::SingleQuoteLowerName<'a>>,
}

pub type Path<'a> = Vec<Name<'a>>;

#[derive(Debug)]
pub enum Name<'a> {
    Upper(token::UpperName<'a>),
    Lower(token::LowerName<'a>),
}

#[derive(Debug)]
pub enum UnderscoreOrLowerName<'a> {
    Underscore(token::Underscore<'a>),
    Lower(token::LowerName<'a>),
}

#[derive(Debug)]
pub enum GrammarSymbolOrSelf<'a> {
    Upper(token::UpperName<'a>),
    DollarUpper(token::DollarUpperName<'a>),
    Self_(token::UpperSelfKw<'a>),
}
