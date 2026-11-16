use super::*;

/// Formats one conflicting table entry and all of its contributing rules.
///
/// `error` must refer to `machine`, which must have been built from `takeoff`.
/// `file_name` is printed as supplied. Lines and character columns start at 1.
/// Both `{:?}` and `{:#?}` produce the same diagnostic.
pub struct TableConflictDiagnostic<'a, 'source> {
    pub error: TableConflictError,
    pub machine: &'a Machine,
    pub takeoff: &'a TakeoffCertificate<'source>,
    pub file_name: &'a str,
}

#[derive(Clone, Copy)]
enum RuleOrigin<'a> {
    Start(token::UpperName<'a>),
    Struct(token::UpperName<'a>),
    EnumVariant {
        enum_name: token::UpperName<'a>,
        variant_name: token::UpperName<'a>,
    },
    ManualVariant {
        manual_name: token::UpperName<'a>,
        function: token::AtLowerName<'a>,
    },
}

impl Debug for TableConflictDiagnostic<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self {
            error,
            machine,
            takeoff,
            file_name: _,
        } = self;
        let state = &machine.states[error.state_id.raw];
        let mut shifts = Vec::new();
        let mut reductions = Vec::new();
        let mut accept = false;
        let shift_symbol = error
            .lookahead
            .into_terminal()
            .map(OriginalSymbolId::from_terminal);

        // Items sort by rule, dot position, then lookahead. Consequently, shift
        // items differing only in lookahead are adjacent and can be collapsed.
        for item in &state.items {
            match item.next_symbol(takeoff) {
                Some(symbol) => {
                    if Some(symbol) == shift_symbol {
                        let key = (item.rule_id, item.dot_index);
                        if shifts.last() != Some(&key) {
                            shifts.push(key);
                        }
                    }
                }
                None => {
                    if item.lookahead != error.lookahead {
                        continue;
                    }
                    match item.rule_id {
                        RuleId::Augmented => accept = true,
                        RuleId::Original(_) => reductions.push(item.rule_id),
                    }
                }
            }
        }

        let kind = match (!shifts.is_empty(), accept, reductions.len()) {
            (true, false, 1) => "shift/reduce",
            (false, false, 2..) => "reduce/reduce",
            (false, true, 1) => "accept/reduce",
            (true, false, 2..) => "shift/reduce and reduce/reduce",
            (false, true, 2..) => "accept/reduce and reduce/reduce",
            _ => unreachable!("The diagnostic must identify a conflicting table entry"),
        };

        write!(f, "error: {kind} conflict")?;
        match error.lookahead {
            EoiOrTerminal::Eoi => write!(f, " at EOI")?,
            EoiOrTerminal::Terminal(id) => {
                write!(f, " on {}", takeoff.preflight.terminal_defs[id].name.raw)?;
            }
        }

        let origins = get_rule_origins(&takeoff.preflight);

        if !shifts.is_empty() {
            let terminal = error
                .lookahead
                .into_terminal()
                .expect("EOI cannot be shifted");
            write!(
                f,
                "\n\n  shift {}",
                takeoff.preflight.terminal_defs[terminal].name.raw
            )?;
            for (rule_id, dot_index) in shifts {
                let origin = origins[rule_id.into_usize()];
                write!(f, "\n    for {origin} at ")?;
                self.write_location(f, origin.anchor())?;
                write!(f, "\n      ")?;
                self.write_rule(f, rule_id, dot_index)?;
            }
        }

        if accept {
            let origin = origins[RuleId::Augmented.into_usize()];
            write!(f, "\n\n  accept {origin} at ")?;
            self.write_location(f, origin.anchor())?;
            write!(f, "\n    start {}", takeoff.preflight.start_symbol.raw)?;
        }

        for rule_id in reductions {
            let origin = origins[rule_id.into_usize()];
            write!(f, "\n\n  reduce {origin} at ")?;
            self.write_location(f, origin.anchor())?;
            write!(f, "\n    ")?;
            self.write_rule(f, rule_id, takeoff[rule_id].rhs.len())?;
        }

        Ok(())
    }
}

impl TableConflictDiagnostic<'_, '_> {
    fn write_rule(&self, f: &mut Formatter<'_>, rule_id: RuleId, dot_index: usize) -> fmt::Result {
        let Rule { lhs, rhs } = &self.takeoff[rule_id];
        let lhs = lhs
            .into_original()
            .expect("The augmented rule is printed as a start item");
        assert!(dot_index <= rhs.len());
        write!(f, "{} <-", self.non_terminal_name(lhs).raw)?;

        for (i, symbol) in rhs.iter().enumerate() {
            if i == dot_index {
                write!(f, " .")?;
            }
            match symbol.as_terminal() {
                Ok(id) => write!(f, " {}", self.takeoff.preflight.terminal_defs[id].name.raw)?,
                Err(id) => write!(f, " {}", self.non_terminal_name(id).raw)?,
            }
        }
        if dot_index == rhs.len() {
            write!(f, " .")?;
        }
        Ok(())
    }

    fn write_location(&self, f: &mut Formatter<'_>, anchor: SourceFragment<'_>) -> fmt::Result {
        let source = self.takeoff.preflight.source;
        let start = FileSource::new(source)
            .substr_range(anchor)
            .expect("Rule origins must belong to the grammar source")
            .start;
        let prefix = &source[..start];
        let line = prefix.bytes().filter(|&b| b == b'\n').count() + 1;
        let column = prefix.rsplit('\n').next().unwrap().chars().count() + 1;
        write!(f, "{}:{line}:{column}", self.file_name)
    }

    fn non_terminal_name(&self, id: OriginalNonTerminalId) -> token::UpperName<'_> {
        match self.takeoff.preflight.non_terminal_defs[id] {
            cst::FileItem::Start(_) => {
                unreachable!("Non-terminal definitions cannot be start items")
            }
            cst::FileItem::Struct(cst::Struct {
                prefix: _,
                name,
                fieldset: _,
            }) => *name,
            cst::FileItem::Enum(cst::Enum {
                prefix: _,
                name,
                variants: _,
            }) => *name,
            cst::FileItem::Manual(cst::Manual {
                name,
                override_: _,
                lifetimes: _,
                variants: _,
            }) => *name,
            cst::FileItem::Terminal(_) => {
                unreachable!("Non-terminal definitions cannot be terminal items")
            }
            cst::FileItem::Bridge(_) => {
                unreachable!("Non-terminal definitions cannot be bridge items")
            }
        }
    }
}

impl Display for RuleOrigin<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Start(name) | Self::Struct(name) => Display::fmt(&name.raw, f),
            Self::EnumVariant {
                enum_name,
                variant_name,
            } => write!(f, "{}::{}", enum_name.raw, variant_name.raw),
            Self::ManualVariant {
                manual_name,
                function,
            } => write!(f, "{}::{}", manual_name.raw, function.raw),
        }
    }
}

impl<'a> RuleOrigin<'a> {
    fn anchor(self) -> SourceFragment<'a> {
        match self {
            Self::Start(name) | Self::Struct(name) => name.raw,
            Self::EnumVariant {
                enum_name: _,
                variant_name,
            } => variant_name.raw,
            Self::ManualVariant {
                manual_name: _,
                function,
            } => function.raw,
        }
    }
}

fn get_rule_origins<'a>(preflight: &PreflightCertificate<'a>) -> Vec<RuleOrigin<'a>> {
    // Match preflight's rule order: augmented first, then declarations and variants.
    let mut out = vec![RuleOrigin::Start(preflight.start_symbol)];
    for item in &preflight.file.items {
        match item {
            cst::FileItem::Start(_) => {}
            cst::FileItem::Struct(cst::Struct {
                prefix: _,
                name,
                fieldset: _,
            }) => out.push(RuleOrigin::Struct(*name)),
            cst::FileItem::Enum(cst::Enum {
                prefix: _,
                name,
                variants,
            }) => {
                for cst::EnumVariant {
                    name: variant_name,
                    fieldset: _,
                } in variants
                {
                    out.push(RuleOrigin::EnumVariant {
                        enum_name: *name,
                        variant_name: *variant_name,
                    });
                }
            }
            cst::FileItem::Manual(cst::Manual {
                name,
                override_: _,
                lifetimes: _,
                variants,
            }) => {
                for cst::ManualVariant {
                    function,
                    inputs: _,
                } in variants
                {
                    out.push(RuleOrigin::ManualVariant {
                        manual_name: *name,
                        function: *function,
                    });
                }
            }
            cst::FileItem::Terminal(_) => {}
            cst::FileItem::Bridge(_) => {}
        }
    }
    out
}
