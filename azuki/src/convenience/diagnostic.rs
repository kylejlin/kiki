use super::*;

pub(super) struct ErrorFormatter<'a> {
    pub source: FileSource<'a>,
    pub file_name: &'a str,
}

impl ErrorFormatter<'_> {
    pub fn lex_error(&self, error: LexError) -> ErrorMessage {
        let LexError {
            character,
            position: TextPosition { byte_index },
        } = error;
        let (message, len) = match character {
            UnexpectedCharOrEndOfInput::EndOfInput => {
                ("unexpected end of input while lexing".to_owned(), 0)
            }
            UnexpectedCharOrEndOfInput::Char(c) => {
                (format!("unexpected character {c:?}"), c.len_utf8())
            }
        };
        self.error(message, (byte_index..byte_index + len).into())
    }

    pub fn parse_error(&self, error: cst::ParseError<'_>) -> ErrorMessage {
        let cst::ParseError { unexpected_token } = error;
        match unexpected_token {
            Some(token) => {
                let fragment = token_fragment(token);
                self.error(
                    format!("unexpected token {:?}", fragment.raw()),
                    self.range(fragment),
                )
            }
            None => self.error("unexpected end of input while parsing", self.eoi()),
        }
    }

    pub fn preflight_error(&self, error: PreflightCheckError<'_>) -> ErrorMessage {
        match error {
            PreflightCheckError::NoStartItem => self.missing_item("start"),
            PreflightCheckError::MultipleStartItems(first, duplicate) => {
                self.duplicate("multiple `start` items", first.raw, duplicate.raw)
            }
            PreflightCheckError::NoTerminalItem => self.missing_item("terminal"),
            PreflightCheckError::MultipleTerminalItems(first, duplicate) => {
                self.duplicate("multiple `terminal` items", first.raw, duplicate.raw)
            }
            PreflightCheckError::NoBridgeItem => self.missing_item("bridge"),
            PreflightCheckError::MultipleBridgeItems(first, duplicate) => {
                self.duplicate("multiple `bridge` items", first, duplicate)
            }
            PreflightCheckError::ItemNameClash(first, duplicate) => self.duplicate(
                format!("duplicate item name `{}`", duplicate.raw),
                first.raw,
                duplicate.raw,
            ),
            PreflightCheckError::EnumVariantNameClash(first, duplicate) => self.duplicate(
                format!("duplicate enum variant name `{}`", duplicate.raw),
                first.raw,
                duplicate.raw,
            ),
            PreflightCheckError::TerminalVariantNameClash(first, duplicate) => self.duplicate(
                format!("duplicate terminal variant name `{}`", duplicate.raw),
                first.raw,
                duplicate.raw,
            ),
            PreflightCheckError::VariantSymbolSequenceClash(first, duplicate) => self.duplicate(
                format!("variant `{duplicate}` has the same grammar symbol sequence as `{first}`"),
                first,
                duplicate,
            ),
            PreflightCheckError::FieldNameClash(first, duplicate) => self.duplicate(
                format!("duplicate field name `{}`", duplicate.raw),
                first.raw,
                duplicate.raw,
            ),
            PreflightCheckError::UndefinedTerminal(name) => self.error(
                format!("undefined terminal `{}`", name.raw),
                self.range(name.raw),
            ),
            PreflightCheckError::UndefinedNonTerminal(name) => self.error(
                format!("undefined non-terminal `{}`", name.raw),
                self.range(name.raw),
            ),
            PreflightCheckError::LifetimeNameClash(first, duplicate) => self.duplicate(
                format!("duplicate lifetime `{}`", duplicate.raw),
                first.raw,
                duplicate.raw,
            ),
            PreflightCheckError::UndefinedLifetime(name) => {
                let mut out = self.error(
                    format!("undefined lifetime `{}`", name.raw),
                    self.range(name.raw),
                );
                out.raw.push_str(
                    "\n\nhelp: Declare this lifetime in the `terminal` item's global lifetime list.",
                );
                out
            }
        }
    }

    fn missing_item(&self, keyword: &str) -> ErrorMessage {
        self.error(
            format!("missing `{keyword}` item; exactly one is required"),
            self.eoi(),
        )
    }

    fn duplicate(
        &self,
        message: impl Display,
        first: SourceFragment<'_>,
        duplicate: SourceFragment<'_>,
    ) -> ErrorMessage {
        let mut out = self.error(message, self.range(duplicate));
        out.raw.push_str(&format!(
            "\n\nnote: first occurrence is here\n{}",
            self.snippet(self.range(first))
        ));
        out
    }

    fn error(&self, message: impl Display, range: Range<usize>) -> ErrorMessage {
        ErrorMessage {
            raw: format!("error: {message}\n{}", self.snippet(range)),
        }
    }

    fn snippet(&self, range: Range<usize>) -> String {
        let source = self.source.raw();
        let prefix = &source[..range.start];
        let line = prefix.bytes().filter(|&b| b == b'\n').count() + 1;
        let column = prefix.rsplit('\n').next().unwrap().chars().count() + 1;
        let line_start = prefix.rfind('\n').map_or(0, |i| i + 1);
        let line_end = source[range.start..]
            .find('\n')
            .map_or(source.len(), |i| range.start + i);
        let line_source = source[line_start..line_end].trim_end_matches('\r');
        let line_end = line_start + line_source.len();
        let marker_start = range.start.min(line_end);
        let marker_end = range.end.min(line_end);
        let text = display_source_text(line_source);
        let padding = display_source_text(&source[line_start..marker_start])
            .chars()
            .count();
        let width = display_source_text(&source[marker_start..marker_end])
            .chars()
            .count()
            .max(1);
        let gutter = " ".repeat(line.to_string().len());
        let padding = " ".repeat(padding);
        let marker = "^".repeat(width);
        let file_name = self.file_name;
        format!(
            "  --> {file_name}:{line}:{column}\n {gutter} |\n {line} | {text}\n {gutter} | {padding}{marker}"
        )
    }

    fn range(&self, fragment: SourceFragment<'_>) -> Range<usize> {
        self.source
            .substr_range(fragment)
            .expect("Diagnostic fragments must belong to the grammar source")
    }

    fn eoi(&self) -> Range<usize> {
        let end = self.source.raw().len();
        (end..end).into()
    }
}

fn token_fragment(token: Token<'_>) -> SourceFragment<'_> {
    match token {
        Token::StartKw(token) => token.raw,
        Token::StructKw(token) => token.raw,
        Token::EnumKw(token) => token.raw,
        Token::ManualKw(token) => token.raw,
        Token::TerminalKw(token) => token.raw,
        Token::BridgeKw(token) => token.raw,
        Token::UpperSelfKw(token) => token.raw,
        Token::Underscore(token) => token.raw,
        Token::LParen(token) => token.raw,
        Token::RParen(token) => token.raw,
        Token::LSquare(token) => token.raw,
        Token::RSquare(token) => token.raw,
        Token::LCurly(token) => token.raw,
        Token::RCurly(token) => token.raw,
        Token::Comma(token) => token.raw,
        Token::DoubleColon(token) => token.raw,
        Token::Colon(token) => token.raw,
        Token::Equal(token) => token.raw,
        Token::Octothorpe(token) => token.raw,
        Token::UpperName(token) => token.raw,
        Token::LowerName(token) => token.raw,
        Token::DollarUpperName(token) => token.raw,
        Token::AtUpperName(token) => token.raw,
        Token::AtLowerName(token) => token.raw,
        Token::SingleQuoteLowerName(token) => token.raw,
        Token::RustStringLiteral(token) => token.raw,
    }
}

fn display_source_text(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '\t' => "    ".to_owned(),
            c if c.is_control() => c.escape_default().to_string(),
            c => c.to_string(),
        })
        .collect()
}
