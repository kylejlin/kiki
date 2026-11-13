use crate::internal_prelude::*;

pub(crate) struct TakeoffCertificate<'a> {
    pub preflight: PreflightCertificate<'a>,

    /// IDs follow terminal declaration order, starting at 1.
    pub terminal_ids: HashMap<SigilFreeStr<'a>, TerminalId>,
}

impl<'a> TakeoffCertificate<'a> {
    pub fn new(preflight: PreflightCertificate<'a>) -> Self {
        let source = preflight.source;
        let terminal = preflight.terminal;
        Self {
            preflight,
            terminal_ids: get_terminal_ids(source, terminal),
        }
    }
}

fn get_terminal_ids<'a>(
    source: &'a str,
    terminal: &m::TerminalEnum,
) -> HashMap<SigilFreeStr<'a>, TerminalId> {
    let m::TerminalEnum {
        prefix: _,
        name: _,
        variants,
    } = terminal;

    variants
        .iter()
        .enumerate()
        .map(|(index, m::TerminalEnumVariant { name, override_: _ })| {
            let key = sigil_free_name(source, name.raw);
            let id = TerminalId {
                raw: NonZeroUsize::new(index + 1).unwrap(),
            };
            (key, id)
        })
        .collect()
}
