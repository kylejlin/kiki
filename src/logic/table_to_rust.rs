use crate::data::{table::*, validated_file::*, KikiErr, RustSrc};
use std::collections::HashSet;

pub fn table_to_rust(table: &Table, file: ValidatedFile) -> Result<RustSrc, KikiErr> {
    let mut used_identifiers = file.used_identifiers;
    let start_src = &file.start;
    let token_src = &file.terminal_enum;
    let token_or_eof_src = create_unique_identifier("TokenOrEof", &mut used_identifiers);
    let token_kind_src = create_unique_identifier("TokenKind", &mut used_identifiers);
    let nonterminal_kind_src = create_unique_identifier("NonterminalKind", &mut used_identifiers);
    let state_src = create_unique_identifier("State", &mut used_identifiers);
    let node_src = create_unique_identifier("Node", &mut used_identifiers);
    let action_src = create_unique_identifier("Action", &mut used_identifiers);
    let rule_kind_src = create_unique_identifier("RuleKind", &mut used_identifiers);

    Ok(RustSrc(format!(
        r#"enum {token_or_eof_src} {{
    Token({token_src}),
    Eof,
}}

{token_kind_enum_def_src}

{nonterminal_kind_enum_def_src}

{state_enum_def_src}

{node_enum_def_src}

enum {action_src} {{
    Shift({state_src}),
    Reduce({rule_kind_src}),
    Accept,
    Err,
}}

{rule_kind_enum_def_src}

pub fn parse<S>(src: S) -> Result<{start_src}, {token_src}>
where S: IntoIterator<Item = {token_src}> {{
    let mut tokens = src.into_iter()
        .map({token_or_eof_src}::Token)
        .chain(std::iter::once({token_or_eof_src}::Eof))
        .peekable();
    let mut states = vec![{state_src}::{INIT_STATE_SRC}];
    let mut nodes: Vec<{node_src}> = vec![];
    loop {{
        let top_state = *states.last().unwrap();
        let top_token_kind = {token_kind_src}::new(tokens.peek().unwrap());
        match get_action(top_state, top_token_kind) {{
            {action_src}::Shift(new_state) => {{
                states.push(new_state);
                nodes.push({node_src}::from_token(tokens.next().unwrap().try_into_token().unwrap()));
            }}

            {action_src}::Reduce(rule_kind) => {{
                let (new_node, new_node_kind) = pop_and_reduce(&mut states, &mut nodes, rule_kind);
                nodes.push(new_node);
                let temp_top_state = *states.last().unwrap();
                let new_state = get_goto(temp_top_state, new_node_kind);
                states.push(new_state);
            }}

            {action_src}::Accept => {{
                return Ok({start_src}::try_from(nodes.pop().unwrap()).unwrap());
            }}

            {action_src}::Err => {{
                return Err(tokens.next().unwrap());
            }}
        }}
    }}
}}

fn pop_and_reduce(states: &mut Vec<{state_src}>, nodes: &mut Vec<{node_src}>, rule_kind: {rule_kind_src}) -> ({node_src}, {nonterminal_kind_src}) {{
    match rule_kind {{
{rule_kind_match_arms_src_indent_2}
    }}
}}"#
    )))
}

const INIT_STATE_SRC: &str = "S0";

fn create_unique_identifier(preferred_name: &str, used: &mut HashSet<String>) -> String {
    if !used.contains(preferred_name) {
        used.insert(preferred_name.to_string());
        return preferred_name.to_string();
    }

    let mut i = 2;
    loop {
        let name = format!("{}{}", preferred_name, i);
        if !used.contains(&name) {
            used.insert(name.clone());
            return name;
        }
        i += 1;
    }
}
