use crate::data::{ast::*, table::*, validated_file::*, KikiErr, RustSrc};
use std::collections::{HashMap, HashSet};

pub fn table_to_rust(table: &Table, file: ValidatedFile) -> Result<RustSrc, KikiErr> {
    let mut used_identifiers = file.defined_identifiers;
    let start_type_name = &file.start;
    let token_enum_name = &file.terminal_enum.name;
    let eof_variant_name = create_unique_identifier("Eof", &mut used_identifiers);
    let quasitoken_enum_name = create_unique_identifier("Quasitoken", &mut used_identifiers);
    let quasitoken_kind_enum_name =
        create_unique_identifier("QuasitokenKind", &mut used_identifiers);
    let nonterminal_kind_enum_name =
        create_unique_identifier("NonterminalKind", &mut used_identifiers);
    let state_enum_name = create_unique_identifier("State", &mut used_identifiers);
    let node_enum_name = create_unique_identifier("Node", &mut used_identifiers);
    let action_enum_name = create_unique_identifier("Action", &mut used_identifiers);
    let rule_kind_enum_name = create_unique_identifier("RuleKind", &mut used_identifiers);

    let node_to_terminal_method_names: HashMap<String, String> = file
        .terminal_enum
        .variants
        .iter()
        .enumerate()
        .map(|(variant_index, variant)| {
            let variant_name_snake_case = pascal_to_snake_case(&variant.dollarless_name);
            let variant_name_original_case = &variant.dollarless_name;
            let method_name = format!("try_into_{variant_name_snake_case}_{variant_index}");
            (variant_name_original_case.to_owned(), method_name)
        })
        .collect();

    let token_kind_enum_variants_indent_1 = file
        .terminal_enum
        .variants
        .iter()
        .map(|variant| {
            let name = &variant.dollarless_name;
            format!("{name},\n")
        })
        .collect::<String>()
        .indent(1);

    let nonterminal_kind_enum_variants_indent_1 = file
        .nonterminals
        .iter()
        .map(|nonterminal| format!("{},\n", nonterminal.name()))
        .collect::<String>()
        .indent(1);

    let state_enum_variants_indent_1 = (0..table.states())
        .map(|i| format!("S{i},"))
        .collect::<String>()
        .indent(1);

    let node_enum_variants_indent_1: String = file
        .nonterminals
        .iter()
        .map(|nonterminal| format!("{name}({name}),", name = nonterminal.name()))
        .chain(file.terminal_enum.variants.iter().map(|variant| {
            let name = &variant.dollarless_name;
            let type_ = &variant.type_;
            format!("{name}({type_}),\n")
        }))
        .collect::<String>()
        .indent(1);

    let rule_kinds = file
        .nonterminals
        .iter()
        .map(|nonterminal| match nonterminal {
            Nonterminal::Struct(_) => 1,
            Nonterminal::Enum(e) => e.variants.len(),
        })
        .sum();
    let rule_kind_enum_variants_indent_1: String = (0..rule_kinds)
        .map(|i| format!("R{i},\n"))
        .collect::<String>()
        .indent(1);

    let pop_and_reduce_match_arms_indent_2: String = file
        .nonterminals
        .iter()
        .flat_map(|nonterminal| match nonterminal {
            Nonterminal::Struct(s) => vec![(s.name.name.to_owned(), &s.fieldset)],
            Nonterminal::Enum(e) => e
                .variants
                .iter()
                .map(|v| {
                    let enum_name = &e.name.name;
                    let variant_name = &v.name.name;
                    (format!("{enum_name}::{variant_name}"), &v.fieldset)
                })
                .collect(),
        })
        .enumerate()
        .map(|(rule_index, (constructor_name, fieldset))| {
            let reduction_code_indent_1: String = match fieldset {
                Fieldset::Empty => constructor_name,
                Fieldset::Named(NamedFieldset { fields }) => todo!(),
                Fieldset::Tuple(TupleFieldset { fields }) => {
                    const ANONYMOUS_FIELD_PREFIX: &str = "t";
                    let child_vars: String = fields
                        .iter()
                        .enumerate()
                        .rev()
                        .map(|(field_index, field)| match field {
                            TupleField::Skipped(_) => "nodes.pop().unwrap();\n".to_owned(),
                            TupleField::Used(IdentOrTerminalIdent::Ident(field_type)) => {
                                let field_type_name = &field_type.name;
                                format!("let {ANONYMOUS_FIELD_PREFIX}{field_index} = {field_type_name}::try_from(nodes.pop().unwrap()).unwrap();\n")
                            },
                            TupleField::Used(IdentOrTerminalIdent::Terminal(field_type)) => {
                                let try_into_method_name = node_to_terminal_method_names.get(&field_type.name).unwrap();
                                format!("let {ANONYMOUS_FIELD_PREFIX}{field_index} = nodes.pop().unwrap().{try_into_method_name}().unwrap();\n")
                            },
                        })
                        .collect();

                    let parent_fields_indent_1: String = fields
                        .iter()
                        .enumerate()
                        .filter_map(|(field_index, field)| match field {
                            TupleField::Skipped(_) => None,
                            TupleField::Used(field_type) => {
                                Some(format!("{ANONYMOUS_FIELD_PREFIX}{field_index},\n"))
                            }
                        })
                        .collect::<String>()
                        .indent(1);

                    format!("{child_vars}{constructor_name}(\n{parent_fields_indent_1})")
                }
            }
            .indent(1);
            format!(
                r#"{rule_kind_enum_name}::R{rule_index} => {{
{reduction_code_indent_1}
}}"#
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
        .indent(2);

    let quasitoken_kind_from_token_match_arms_indent_3: String = file
        .terminal_enum
        .variants
        .iter()
        .map(|variant| {
            let name = &variant.dollarless_name;
            format!("{token_enum_name}::{name}(_) => Self::{name},\n")
        })
        .collect::<String>()
        .indent(3);

    let node_from_token_match_arms_indent_3: String = file
        .terminal_enum
        .variants
        .iter()
        .map(|variant| {
            let name = &variant.dollarless_name;
            format!("{token_enum_name}::{name}(t) => Self::{name}(t),\n")
        })
        .collect::<String>()
        .indent(3);

    let impl_try_from_node_for_each_nonterminal: String = file
        .nonterminals
        .iter()
        .map(|nonterminal| {
            let nonterminal_name = nonterminal.name();
            format!(
                r#"impl TryFrom<{node_enum_name}> for {nonterminal_name} {{
    type Error = {node_enum_name};

    fn try_from(node: {node_enum_name}) -> Result<Self, Self::Error> {{
        match node {{
            {node_enum_name}::{nonterminal_name}(n) => Ok(n),
            _ => Err(node),
        }}
    }}
}}"#
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    let node_try_into_terminal_variant_name_variant_index_fns_indent_1: String = file
        .terminal_enum
        .variants
        .iter()
        .enumerate()
        .map(|(variant_index, variant)| {
            let variant_name_original_case = &variant.dollarless_name;
            let method_name = node_to_terminal_method_names
                .get(variant_name_original_case)
                .unwrap();
            let type_ = &variant.type_;
            format!(
                r#"fn {method_name}(self) -> Result<{type_}, Self> {{
    match self {{
        Self::{variant_name_original_case}(t) => Ok(t),
        _ => Err(self),
    }}
}}"#
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n")
        .indent(1);

    Ok(RustSrc(format!(
        r#"// This code was generated by Kiki.

enum {quasitoken_enum_name} {{
    Token({token_enum_name}),
    {eof_variant_name},
}}

#[derive(Clone, Copy)]
enum {quasitoken_kind_enum_name} {{
{token_kind_enum_variants_indent_1}
    {eof_variant_name},
}}

#[derive(Clone, Copy)]
enum {nonterminal_kind_enum_name} {{
{nonterminal_kind_enum_variants_indent_1}
}}

#[derive(Clone, Copy)]
enum {state_enum_name} {{
{state_enum_variants_indent_1}
}}

enum {node_enum_name} {{
{node_enum_variants_indent_1}
}}

#[derive(Clone, Copy)]
enum {action_enum_name} {{
    Shift({state_enum_name}),
    Reduce({rule_kind_enum_name}),
    Accept,
    Err,
}}

#[derive(Clone, Copy)]
enum {rule_kind_enum_name} {{
{rule_kind_enum_variants_indent_1}
}}

pub fn parse<S>(src: S) -> Result<{start_type_name}, {token_enum_name}>
where S: IntoIterator<Item = {token_enum_name}> {{
    let mut tokens = src.into_iter()
        .map({quasitoken_enum_name}::Token)
        .chain(std::iter::once({quasitoken_enum_name}::Eof))
        .peekable();
    let mut states = vec![{state_enum_name}::S0];
    let mut nodes: Vec<{node_enum_name}> = vec![];
    loop {{
        let top_state = *states.last().unwrap();
        let next_quasitoken_kind = {quasitoken_kind_enum_name}::from_quasitoken(tokens.peek().unwrap());
        match get_action(top_state, next_quasitoken_kind) {{
            {action_enum_name}::Shift(new_state) => {{
                states.push(new_state);
                nodes.push({node_enum_name}::from_token(tokens.next().unwrap().try_into_token().unwrap()));
            }}

            {action_enum_name}::Reduce(rule_kind) => {{
                let (new_node, new_node_kind) = pop_and_reduce(&mut states, &mut nodes, rule_kind);
                nodes.push(new_node);
                let temp_top_state = *states.last().unwrap();
                let Some(new_state) = get_goto(temp_top_state, new_node_kind) else {{
                    return Err(tokens.next().unwrap());
                }};
                states.push(new_state);
            }}

            {action_enum_name}::Accept => {{
                return Ok({start_type_name}::try_from(nodes.pop().unwrap()).unwrap());
            }}

            {action_enum_name}::Err => {{
                return Err(tokens.next().unwrap());
            }}
        }}
    }}
}}

fn pop_and_reduce(states: &mut Vec<{state_enum_name}>, nodes: &mut Vec<{node_enum_name}>, rule_kind: {rule_kind_enum_name}) -> ({node_enum_name}, {nonterminal_kind_enum_name}) {{
    match rule_kind {{
{pop_and_reduce_match_arms_indent_2}
    }}
}}

impl {quasitoken_kind_enum_name} {{
    fn from_quasitoken(quasitoken: &{quasitoken_enum_name}) -> Self {{
        match quasitoken {{
            Self::Token(token) => Self::from_token(token),
            Self::{eof_variant_name} => Self::{eof_variant_name},
        }}
    }}

    fn from_token(token: &{token_enum_name}) -> Self {{
        match token {{
{quasitoken_kind_from_token_match_arms_indent_3}
        }}
    }}
}}

impl {node_enum_name} {{
    fn from_token(token: {token_enum_name}) -> Self {{
        match token {{
{node_from_token_match_arms_indent_3}
        }}
        }}
    }}
}}

impl {quasitoken_enum_name} {{
    fn try_into_token(self) -> Result<{token_enum_name}, ()> {{
        match self {{
            Self::Token(token) => Ok(token),
            Self::{eof_variant_name} => Err(()),
        }}
    }}
}}

fn get_action(top_state: {state_enum_name}, next_quasitoken_kind: {quasitoken_kind_enum_name}) -> {action_enum_name} {{
    todo!()
}}

fn get_goto(top_state: {state_enum_name}, new_node_kind: {nonterminal_kind_enum_name}) -> Option<{state_enum_name}> {{
    todo!()
}}

{impl_try_from_node_for_each_nonterminal}

impl {node_enum_name} {{
{node_try_into_terminal_variant_name_variant_index_fns_indent_1}
}}

// TODO: Add nonterminal type definitions.
"#
    )))
}

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

trait Indent {
    fn indent(&self, indent: usize) -> String;
}

impl Indent for str {
    fn indent(&self, level: usize) -> String {
        let mut out = String::new();
        let indent = &"    ".repeat(level);
        out.push_str(indent);
        for c in self.chars() {
            out.push(c);
            if c == '\n' {
                out.push_str(indent);
            }
        }
        out
    }
}

fn pascal_to_snake_case(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars().fuse();

    if let Some(c) = s.chars().next() {
        out.push(c.to_ascii_lowercase());
    }

    for c in chars {
        if c.is_uppercase() {
            out.push('_');
        }
        out.push(c.to_ascii_lowercase());
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    mod pascal_to_snake_case {
        use super::*;

        #[test]
        fn lower_x() {
            assert_eq!("x", pascal_to_snake_case("x"));
        }

        #[test]
        fn upper_x() {
            assert_eq!("x", pascal_to_snake_case("X"));
        }

        #[test]
        fn lower_hi() {
            assert_eq!("hi", pascal_to_snake_case("hi"));
        }

        #[test]
        fn titlecase_hi() {
            assert_eq!("hi", pascal_to_snake_case("Hi"));
        }

        #[test]
        fn uppercase_hi() {
            assert_eq!("h_i", pascal_to_snake_case("HI"));
        }

        #[test]
        fn titlecase_hi_there() {
            assert_eq!("hi_there", pascal_to_snake_case("HiThere"));
        }

        #[test]
        fn uppercase_hi_titlecase_there() {
            assert_eq!("h_i_there", pascal_to_snake_case("HIThere"));
        }
    }
}
