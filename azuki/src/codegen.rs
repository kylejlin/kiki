use crate::internal_prelude::*;

pub(crate) struct CodeGenerator<'a> {
    pub takeoff: &'a TakeoffCertificate<'a>,
    pub machine: &'a Machine,
    pub table: &'a Table,
}

impl CodeGenerator<'_> {
    pub fn codegen(self) -> RustCode {
        SrcBuilder::new(self).file_src()
    }
}

struct SrcBuilder<'a> {
    generator: CodeGenerator<'a>,
    names: GeneratedNames,
    terminal_lifetimes: LifetimeSet,
    node_lifetimes: LifetimeSet,
}

struct GeneratedNames {
    bridge: String,
    eoi_variant: String,
    eoi_or_terminal: String,
    eoi_or_terminal_kind: String,
    non_terminal_kind: String,
    state: String,
    node: String,
    action: String,
    rule_kind: String,
    reduce: String,
    action_table: String,
    goto_table: String,
    source_type: String,
}

enum Reduction<'a> {
    Construct {
        constructor: String,
        fieldset: &'a cst::Fieldset<'a>,
    },
    Call {
        function: &'a str,
        inputs: &'a [cst::TupleField<'a>],
    },
}

#[derive(Clone, Copy)]
struct FieldsetOptions {
    is_struct: bool,
    indent: usize,
}

impl<'a> SrcBuilder<'a> {
    fn new(generator: CodeGenerator<'a>) -> Self {
        let preflight = &generator.takeoff.preflight;
        let mut used = HashSet::new();
        used.insert(preflight.terminal_enum.name.raw.raw().to_owned());
        for (_, item) in preflight.non_terminal_defs.enumerate() {
            used.insert(non_terminal_name(item).to_owned());
        }
        for (_, variant) in preflight.terminal_defs.enumerate() {
            used.insert(variant.name.raw.raw()[1..].to_owned());
        }

        let names = GeneratedNames {
            bridge: create_unique_identifier("__azuki_bridge", &mut used),
            eoi_variant: create_unique_identifier("Eoi", &mut used),
            eoi_or_terminal: create_unique_identifier("EoiOrTerminal", &mut used),
            eoi_or_terminal_kind: create_unique_identifier("EoiOrTerminalKind", &mut used),
            non_terminal_kind: create_unique_identifier("NonTerminalKind", &mut used),
            state: create_unique_identifier("State", &mut used),
            node: create_unique_identifier("Node", &mut used),
            action: create_unique_identifier("Action", &mut used),
            rule_kind: create_unique_identifier("RuleKind", &mut used),
            reduce: create_unique_identifier("reduce", &mut used),
            action_table: create_unique_identifier("ACTION_TABLE", &mut used),
            goto_table: create_unique_identifier("GOTO_TABLE", &mut used),
            source_type: create_unique_identifier("S", &mut used),
        };

        let mut terminal_lifetimes = LifetimeSet::empty();
        for (_, lifetimes) in generator.takeoff.terminal_lifetime_sets.enumerate() {
            terminal_lifetimes |= lifetimes;
        }
        let mut node_lifetimes = LifetimeSet::empty();
        node_lifetimes |= &terminal_lifetimes;
        for (_, lifetimes) in generator.takeoff.non_terminal_lifetime_sets.enumerate() {
            node_lifetimes |= lifetimes;
        }

        Self {
            generator,
            names,
            terminal_lifetimes,
            node_lifetimes,
        }
    }

    fn file_src(&self) -> RustCode {
        let preflight = &self.generator.takeoff.preflight;
        let GeneratedNames {
            bridge,
            eoi_variant,
            eoi_or_terminal,
            eoi_or_terminal_kind,
            non_terminal_kind,
            state,
            node,
            action,
            rule_kind,
            reduce: _,
            action_table,
            goto_table,
            source_type,
        } = &self.names;
        let grammar_hash = sha256::digest(preflight.source);
        let bridge_path = preflight
            .bridge_path
            .iter()
            .map(|name| match name {
                cst::Name::Upper(name) => name.raw.raw(),
                cst::Name::Lower(name) => name.raw.raw(),
            })
            .collect::<Vec<_>>()
            .join("::");
        let terminal_prefix = prefix_src(preflight.terminal_enum.prefix);
        let terminal = preflight.terminal_enum.name.raw.raw();
        let terminal_lifetimes = self.lifetime_list_src(&self.terminal_lifetimes);
        let node_lifetimes = self.lifetime_list_src(&self.node_lifetimes);
        let mut parse_parameters = self.lifetime_names(&self.node_lifetimes);
        parse_parameters.push(source_type.to_owned());
        let parse_parameters = generic_list_src(&parse_parameters);
        let start_id = preflight.rules[0].rhs[0]
            .as_non_terminal()
            .expect("The augmented rule contains the original start symbol");
        let start_type = self.non_terminal_type_src(start_id, false);
        let start_extraction = self.extraction_method_name(start_id.into());
        let start_state = self.generator.machine.start_id.raw;
        let state_count = self.generator.machine.states.len();
        let terminal_count = preflight.terminal_defs.len() + 1;
        let non_terminal_count = preflight.non_terminal_defs.len();
        let terminal_variants = self.terminal_variants_src();
        let non_terminal_defs = self.non_terminal_defs_src();
        let terminal_kind_variants = self.terminal_kind_variants_src();
        let non_terminal_kind_variants = self.non_terminal_kind_variants_src();
        let state_variants = (0..state_count)
            .map(|id| format!("    S{id} = {id},"))
            .collect::<Vec<_>>()
            .join("\n");
        let node_variants = self.node_variants_src();
        let rule_variants = (1..preflight.rules.len())
            .map(|id| format!("    R{id} = {id},"))
            .collect::<Vec<_>>()
            .join("\n");
        let reductions = self.reductions_src();
        let dispatch_arms = (1..preflight.rules.len())
            .map(|id| {
                let function = self.reduce_function_name(RuleId::from_usize(id));
                format!("        {rule_kind}::R{id} => {function}(states, nodes),")
            })
            .collect::<Vec<_>>()
            .join("\n");
        let terminal_kind_arms = preflight
            .terminal_defs
            .enumerate()
            .map(|(_, variant)| {
                let name = &variant.name.raw.raw()[1..];
                format!("            {terminal}::{name}(_) => Self::{name},")
            })
            .collect::<Vec<_>>()
            .join("\n");
        let terminal_match_value = if preflight.terminal_defs.len() == 0 {
            "*terminal"
        } else {
            "terminal"
        };
        let from_terminal_arms = preflight
            .terminal_defs
            .enumerate()
            .map(|(_, variant)| {
                let name = &variant.name.raw.raw()[1..];
                format!("            {terminal}::{name}(value) => Self::Terminal{name}(value),")
            })
            .collect::<Vec<_>>()
            .join("\n");
        let extraction_methods = self.extraction_methods_src();
        let action_rows = self.action_rows_src();
        let goto_rows = self.goto_rows_src();

        RustCode {
            raw: format!(
                r#"// This code was generated by Azuki.
// @sha256 {grammar_hash}

#![allow(non_snake_case)]
#![allow(dead_code)]

use {bridge_path} as {bridge};

{terminal_prefix}pub enum {terminal}{terminal_lifetimes} {{
{terminal_variants}
}}

pub mod non_terminals {{
{non_terminal_defs}
}}

/// Returns `Err(Some(token))` for an unexpected token, or `Err(None)` for unexpected end of input.
pub fn parse{parse_parameters}(src: {source_type}) -> ::std::result::Result<{start_type}, ::std::option::Option<{terminal}{terminal_lifetimes}>>
where
    {source_type}: ::std::iter::IntoIterator<Item = {terminal}{terminal_lifetimes}>,
{{
    let mut input = src.into_iter()
        .map({eoi_or_terminal}::Terminal)
        .chain(::std::iter::once({eoi_or_terminal}::{eoi_variant}))
        .peekable();
    let mut states = ::std::vec![{state}::S{start_state}];
    let mut nodes: ::std::vec::Vec<{node}{node_lifetimes}> = ::std::vec![];
    loop {{
        let top_state = *states.last().unwrap();
        let next_kind = {eoi_or_terminal_kind}::from_eoi_or_terminal(input.peek().unwrap());
        match get_action(top_state, next_kind) {{
            {action}::Shift(new_state) => {{
                states.push(new_state);
                nodes.push({node}::from_terminal(input.next().unwrap().try_into_terminal().unwrap()));
            }}
            {action}::Reduce(rule) => {{
                let (new_node, new_kind) = pop_and_reduce(&mut states, &mut nodes, rule);
                nodes.push(new_node);
                let top_state = *states.last().unwrap();
                let ::std::option::Option::Some(new_state) = get_goto(top_state, new_kind) else {{
                    return ::std::result::Result::Err(input.next().unwrap().try_into_terminal().ok());
                }};
                states.push(new_state);
            }}
            {action}::Accept => {{
                return ::std::result::Result::Ok(nodes.pop().unwrap().{start_extraction}().ok().unwrap());
            }}
            {action}::Err => {{
                return ::std::result::Result::Err(input.next().unwrap().try_into_terminal().ok());
            }}
        }}
    }}
}}

enum {eoi_or_terminal}{terminal_lifetimes} {{
    {eoi_variant},
    Terminal({terminal}{terminal_lifetimes}),
}}

#[derive(Clone, Copy, Debug)]
enum {eoi_or_terminal_kind} {{
    {eoi_variant} = 0,
{terminal_kind_variants}
}}

#[derive(Clone, Copy, Debug)]
enum {non_terminal_kind} {{
{non_terminal_kind_variants}
}}

#[derive(Clone, Copy, Debug)]
enum {state} {{
{state_variants}
}}

enum {node}{node_lifetimes} {{
{node_variants}
}}

#[derive(Clone, Copy, Debug)]
enum {action} {{
    Shift({state}),
    Reduce({rule_kind}),
    Accept,
    Err,
}}

#[derive(Clone, Copy, Debug)]
enum {rule_kind} {{
{rule_variants}
}}

fn pop_and_reduce{node_lifetimes}(states: &mut ::std::vec::Vec<{state}>, nodes: &mut ::std::vec::Vec<{node}{node_lifetimes}>, rule: {rule_kind}) -> ({node}{node_lifetimes}, {non_terminal_kind}) {{
    match rule {{
{dispatch_arms}
    }}
}}

{reductions}

impl {eoi_or_terminal_kind} {{
    fn from_eoi_or_terminal{terminal_lifetimes}(value: &{eoi_or_terminal}{terminal_lifetimes}) -> Self {{
        match value {{
            {eoi_or_terminal}::{eoi_variant} => Self::{eoi_variant},
            {eoi_or_terminal}::Terminal(terminal) => Self::from_terminal(terminal),
        }}
    }}

    fn from_terminal{terminal_lifetimes}(terminal: &{terminal}{terminal_lifetimes}) -> Self {{
        match {terminal_match_value} {{
{terminal_kind_arms}
        }}
    }}
}}

impl{node_lifetimes} {node}{node_lifetimes} {{
    fn from_terminal(terminal: {terminal}{terminal_lifetimes}) -> Self {{
        match terminal {{
{from_terminal_arms}
        }}
    }}

{extraction_methods}
}}

impl{terminal_lifetimes} {eoi_or_terminal}{terminal_lifetimes} {{
    fn try_into_terminal(self) -> ::std::result::Result<{terminal}{terminal_lifetimes}, ()> {{
        match self {{
            Self::Terminal(value) => ::std::result::Result::Ok(value),
            Self::{eoi_variant} => ::std::result::Result::Err(()),
        }}
    }}
}}

static {action_table}: [[{action}; {terminal_count}]; {state_count}] = [
{action_rows}
];

fn get_action(state: {state}, next_kind: {eoi_or_terminal_kind}) -> {action} {{
    {action_table}[state as usize][next_kind as usize]
}}

static {goto_table}: [[::std::option::Option<{state}>; {non_terminal_count}]; {state_count}] = [
{goto_rows}
];

fn get_goto(state: {state}, new_kind: {non_terminal_kind}) -> ::std::option::Option<{state}> {{
    {goto_table}[state as usize][new_kind as usize]
}}
"#
            ),
        }
    }

    fn terminal_variants_src(&self) -> String {
        self.generator
            .takeoff
            .preflight
            .terminal_defs
            .enumerate()
            .map(|(id, variant)| {
                let name = &variant.name.raw.raw()[1..];
                let type_ = self.terminal_type_src(id, false);
                format!("    {name}({type_}),")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn non_terminal_defs_src(&self) -> String {
        let takeoff = self.generator.takeoff;
        takeoff
            .preflight
            .non_terminal_defs
            .enumerate()
            .map(|(id, item)| {
                let rule_ids = &takeoff.lhs_to_rule_ids[id];
                let lifetimes = self.lifetime_list_src(&takeoff.non_terminal_lifetime_sets[id]);
                match item {
                    cst::FileItem::Start(_) => unreachable!("Start items are not non-terminals"),
                    cst::FileItem::Struct(cst::Struct {
                        prefix,
                        name,
                        fieldset,
                    }) => {
                        let prefix = prefix_src(*prefix);
                        let name = name.raw.raw();
                        let fieldset = self.fieldset_src(
                            fieldset,
                            &takeoff[rule_ids[0]].rhs,
                            FieldsetOptions {
                                is_struct: true,
                                indent: 1,
                            },
                        );
                        format!("{prefix}    pub struct {name}{lifetimes}{fieldset}")
                    }
                    cst::FileItem::Enum(cst::Enum {
                        prefix,
                        name,
                        variants,
                    }) => {
                        let prefix = prefix_src(*prefix);
                        let name = name.raw.raw();
                        let variants = variants
                            .iter()
                            .zip(rule_ids)
                            .map(|(cst::EnumVariant { name, fieldset }, &rule)| {
                                let name = name.raw.raw();
                                let fieldset = self.fieldset_src(
                                    fieldset,
                                    &takeoff[rule].rhs,
                                    FieldsetOptions {
                                        is_struct: false,
                                        indent: 2,
                                    },
                                );
                                format!("        {name}{fieldset},")
                            })
                            .collect::<Vec<_>>()
                            .join("\n");
                        format!("{prefix}    pub enum {name}{lifetimes} {{\n{variants}\n    }}")
                    }
                    cst::FileItem::Manual(cst::Manual {
                        name,
                        override_,
                        lifetimes: _,
                        variants: _,
                    }) => {
                        let name = name.raw.raw();
                        let target = override_.map_or(name, |name| &name.raw.raw()[1..]);
                        let bridge = &self.names.bridge;
                        format!("    pub use super::{bridge}::{target} as {name};")
                    }
                    cst::FileItem::Terminal(_) => {
                        unreachable!("Terminal items are not non-terminals")
                    }
                    cst::FileItem::Bridge(_) => unreachable!("Bridge items are not non-terminals"),
                }
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    fn fieldset_src(
        &self,
        fieldset: &cst::Fieldset<'_>,
        rhs: &[OriginalSymbolId],
        options: FieldsetOptions,
    ) -> String {
        let FieldsetOptions { is_struct, indent } = options;
        let empty = if is_struct { ";" } else { "" };
        let pub_ = if is_struct { "pub " } else { "" };
        let field_indent = "    ".repeat(indent + 1);
        let closing_indent = "    ".repeat(indent);
        match fieldset {
            cst::Fieldset::Empty => empty.to_owned(),
            cst::Fieldset::Named(cst::NamedFieldset { fields }) => {
                let fields = fields
                    .iter()
                    .zip(rhs)
                    .filter_map(
                        |(cst::NamedField { name, symbol: _ }, &symbol)| match name {
                            cst::UnderscoreOrLowerName::Underscore(_) => None,
                            cst::UnderscoreOrLowerName::Lower(name) => {
                                let name = name.raw.raw();
                                let type_ = self.symbol_type_src(symbol, true);
                                Some(format!("{field_indent}{pub_}{name}: {type_},"))
                            }
                        },
                    )
                    .collect::<Vec<_>>();
                if fields.is_empty() {
                    return empty.to_owned();
                }
                format!(" {{\n{}\n{closing_indent}}}", fields.join("\n"))
            }
            cst::Fieldset::Tuple(cst::TupleFieldset { fields }) => {
                let fields = fields
                    .iter()
                    .zip(rhs)
                    .filter_map(|(field, &symbol)| match field {
                        cst::TupleField::Used(_) => {
                            let type_ = self.symbol_type_src(symbol, true);
                            Some(format!("{field_indent}{pub_}{type_},"))
                        }
                        cst::TupleField::Skipped(_) => None,
                    })
                    .collect::<Vec<_>>();
                if fields.is_empty() {
                    return empty.to_owned();
                }
                format!("(\n{}\n{closing_indent}){empty}", fields.join("\n"))
            }
        }
    }

    fn terminal_kind_variants_src(&self) -> String {
        self.generator
            .takeoff
            .preflight
            .terminal_defs
            .enumerate()
            .map(|(id, variant)| {
                let name = &variant.name.raw.raw()[1..];
                let index = EoiOrTerminal::Terminal(id).into_usize();
                format!("    {name} = {index},")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn non_terminal_kind_variants_src(&self) -> String {
        self.generator
            .takeoff
            .preflight
            .non_terminal_defs
            .enumerate()
            .map(|(id, item)| {
                let name = non_terminal_name(item);
                let index = id.zero_based_index();
                format!("    {name} = {index},")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn node_variants_src(&self) -> String {
        let preflight = &self.generator.takeoff.preflight;
        preflight
            .non_terminal_defs
            .enumerate()
            .map(|(id, item)| {
                let name = non_terminal_name(item);
                let type_ = self.non_terminal_type_src(id, false);
                format!("    NonTerminal{name}({type_}),")
            })
            .chain(preflight.terminal_defs.enumerate().map(|(id, variant)| {
                let name = &variant.name.raw.raw()[1..];
                let type_ = self.terminal_type_src(id, false);
                format!("    Terminal{name}({type_}),")
            }))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn reductions_src(&self) -> String {
        let takeoff = self.generator.takeoff;
        let mut functions = Vec::new();
        for (id, item) in takeoff.preflight.non_terminal_defs.enumerate() {
            let rule_ids = &takeoff.lhs_to_rule_ids[id];
            match item {
                cst::FileItem::Start(_) => unreachable!("Start items are not non-terminals"),
                cst::FileItem::Struct(cst::Struct {
                    prefix: _,
                    name,
                    fieldset,
                }) => {
                    functions.push(self.reduce_function_src(
                        rule_ids[0],
                        Reduction::Construct {
                            constructor: format!("non_terminals::{}", name.raw.raw()),
                            fieldset,
                        },
                    ));
                }
                cst::FileItem::Enum(cst::Enum {
                    prefix: _,
                    name,
                    variants,
                }) => {
                    for (
                        cst::EnumVariant {
                            name: variant,
                            fieldset,
                        },
                        &rule,
                    ) in variants.iter().zip(rule_ids)
                    {
                        functions.push(self.reduce_function_src(
                            rule,
                            Reduction::Construct {
                                constructor: format!(
                                    "non_terminals::{}::{}",
                                    name.raw.raw(),
                                    variant.raw.raw(),
                                ),
                                fieldset,
                            },
                        ));
                    }
                }
                cst::FileItem::Manual(cst::Manual {
                    name: _,
                    override_: _,
                    lifetimes: _,
                    variants,
                }) => {
                    for (cst::ManualVariant { function, inputs }, &rule) in
                        variants.iter().zip(rule_ids)
                    {
                        functions.push(self.reduce_function_src(
                            rule,
                            Reduction::Call {
                                function: &function.raw.raw()[1..],
                                inputs,
                            },
                        ));
                    }
                }
                cst::FileItem::Terminal(_) => unreachable!("Terminal items are not non-terminals"),
                cst::FileItem::Bridge(_) => unreachable!("Bridge items are not non-terminals"),
            }
        }
        functions.join("\n\n")
    }

    fn reduce_function_src(&self, rule_id: RuleId, reduction: Reduction<'_>) -> String {
        let rule = &self.generator.takeoff[rule_id];
        let lhs = rule
            .lhs
            .into_original()
            .expect("Only original rules are reduced");
        let name = non_terminal_name(self.generator.takeoff.preflight.non_terminal_defs[lhs]);
        let (variables, value) = match reduction {
            Reduction::Construct {
                constructor,
                fieldset,
            } => match fieldset {
                cst::Fieldset::Empty => (Vec::new(), constructor),
                cst::Fieldset::Named(cst::NamedFieldset { fields }) => {
                    let variables: Vec<_> = fields
                        .iter()
                        .enumerate()
                        .map(|(i, field)| {
                            let cst::NamedField { name, symbol: _ } = field;
                            match name {
                                cst::UnderscoreOrLowerName::Underscore(_) => None,
                                cst::UnderscoreOrLowerName::Lower(name) => {
                                    Some(format!("{}_{i}", name.raw.raw()))
                                }
                            }
                        })
                        .collect();
                    let fields = fields
                        .iter()
                        .zip(&variables)
                        .filter_map(|(field, variable)| {
                            let cst::NamedField { name, symbol: _ } = field;
                            let cst::UnderscoreOrLowerName::Lower(name) = name else {
                                return None;
                            };
                            Some(format!(
                                "{}: {}",
                                name.raw.raw(),
                                variable.as_ref().unwrap()
                            ))
                        })
                        .collect::<Vec<_>>();
                    let value = if fields.is_empty() {
                        constructor
                    } else {
                        format!("{constructor} {{ {} }}", fields.join(", "))
                    };
                    (variables, value)
                }
                cst::Fieldset::Tuple(cst::TupleFieldset { fields }) => {
                    let variables = tuple_variables(fields);
                    let arguments = variables.iter().flatten().cloned().collect::<Vec<_>>();
                    let value = if arguments.is_empty() {
                        constructor
                    } else {
                        format!("{constructor}({})", arguments.join(", "))
                    };
                    (variables, value)
                }
            },
            Reduction::Call { function, inputs } => {
                let variables = tuple_variables(inputs);
                let arguments = variables
                    .iter()
                    .flatten()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ");
                let bridge = &self.names.bridge;
                (variables, format!("{bridge}::{function}({arguments})"))
            }
        };
        assert_eq!(
            variables.len(),
            rule.rhs.len(),
            "Reduction fields must match their rule's RHS"
        );
        let mut pop = String::new();
        for (variable, &symbol) in variables.iter().zip(&rule.rhs).rev() {
            match variable {
                Some(variable) => {
                    let method = self.extraction_method_name(symbol);
                    pop.push_str(&format!(
                        "    let {variable} = nodes.pop().unwrap().{method}().ok().unwrap();\n"
                    ));
                }
                None => pop.push_str("    nodes.pop().unwrap();\n"),
            }
        }
        let rhs_len = rule.rhs.len();
        let (states_parameter, nodes_parameter) = if rhs_len == 0 {
            ("_states", "_nodes")
        } else {
            pop.push_str(&format!(
                "    states.truncate(states.len() - {rhs_len});\n\n"
            ));
            ("states", "nodes")
        };
        let function = self.reduce_function_name(rule_id);
        let lifetimes = self.lifetime_list_src(&self.node_lifetimes);
        let state = &self.names.state;
        let node = &self.names.node;
        let kind = &self.names.non_terminal_kind;
        format!(
            r#"fn {function}{lifetimes}({states_parameter}: &mut ::std::vec::Vec<{state}>, {nodes_parameter}: &mut ::std::vec::Vec<{node}{lifetimes}>) -> ({node}{lifetimes}, {kind}) {{
{pop}    (
        {node}::NonTerminal{name}({value}),
        {kind}::{name},
    )
}}"#
        )
    }

    fn reduce_function_name(&self, rule: RuleId) -> String {
        format!("{}_r{}", self.names.reduce, rule.into_usize())
    }

    fn extraction_methods_src(&self) -> String {
        let preflight = &self.generator.takeoff.preflight;
        let non_terminals = preflight
            .non_terminal_defs
            .enumerate()
            .map(|(id, _)| id.into());
        let terminals = preflight.terminal_defs.enumerate().map(|(id, _)| id.into());
        non_terminals
            .chain(terminals)
            .map(|symbol| {
                let method = self.extraction_method_name(symbol);
                let type_ = self.symbol_type_src(symbol, false);
                let variant = match symbol.as_terminal() {
                    Ok(id) => format!(
                        "Terminal{}",
                        &preflight.terminal_defs[id].name.raw.raw()[1..]
                    ),
                    Err(id) => format!(
                        "NonTerminal{}",
                        non_terminal_name(preflight.non_terminal_defs[id])
                    ),
                };
                format!(
                    r#"    fn {method}(self) -> ::std::result::Result<{type_}, Self> {{
        match self {{
            Self::{variant}(value) => ::std::result::Result::Ok(value),
            _ => ::std::result::Result::Err(self),
        }}
    }}"#
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    fn extraction_method_name(&self, symbol: OriginalSymbolId) -> String {
        let preflight = &self.generator.takeoff.preflight;
        match symbol.as_terminal() {
            Ok(id) => {
                let name = &preflight.terminal_defs[id].name.raw.raw()[1..];
                let index = EoiOrTerminal::Terminal(id).into_usize();
                format!("try_into_terminal_{}_{index}", pascal_to_snake_case(name))
            }
            Err(id) => {
                let name = non_terminal_name(preflight.non_terminal_defs[id]);
                let index = id.zero_based_index() + 1;
                format!(
                    "try_into_non_terminal_{}_{index}",
                    pascal_to_snake_case(name)
                )
            }
        }
    }

    fn action_rows_src(&self) -> String {
        let terminal_count = self.generator.takeoff.preflight.terminal_defs.len() + 1;
        let action = &self.names.action;
        let state = &self.names.state;
        let rule_kind = &self.names.rule_kind;
        self.generator
            .machine
            .enumerate_states()
            .map(|(state_id, _)| {
                let entries = (0..terminal_count)
                    .map(|index| {
                        let lookahead = EoiOrTerminal::from_usize(index);
                        match self.generator.table[(state_id, lookahead)] {
                            Action::Shift(id) => format!("{action}::Shift({state}::S{})", id.raw),
                            Action::Reduce(RuleId::Augmented) => {
                                unreachable!("The augmented rule accepts instead of reducing")
                            }
                            Action::Reduce(id) => {
                                format!("{action}::Reduce({rule_kind}::R{})", id.into_usize())
                            }
                            Action::Accept => format!("{action}::Accept"),
                            Action::Err => format!("{action}::Err"),
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("    [{entries}],")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn goto_rows_src(&self) -> String {
        let state = &self.names.state;
        self.generator
            .machine
            .enumerate_states()
            .map(|(state_id, _)| {
                let entries = self
                    .generator
                    .takeoff
                    .preflight
                    .non_terminal_defs
                    .enumerate()
                    .map(|(id, _)| match self.generator.table[(state_id, id)] {
                        Goto::State(id) => {
                            format!("::std::option::Option::Some({state}::S{})", id.raw)
                        }
                        Goto::Err => "::std::option::Option::None".to_owned(),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("    [{entries}],")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn symbol_type_src(&self, symbol: OriginalSymbolId, in_non_terminals: bool) -> String {
        match symbol.as_terminal() {
            Ok(id) => self.terminal_type_src(id, in_non_terminals),
            Err(id) => self.non_terminal_type_src(id, in_non_terminals),
        }
    }

    fn terminal_type_src(&self, id: TerminalId, in_non_terminals: bool) -> String {
        let cst::TerminalEnumVariant {
            name,
            override_,
            lifetimes,
        } = self.generator.takeoff.preflight.terminal_defs[id];
        let target = override_.map_or(&name.raw.raw()[1..], |name| &name.raw.raw()[1..]);
        let bridge = &self.names.bridge;
        let parent = if in_non_terminals { "super::" } else { "" };
        let lifetimes = declared_lifetime_list_src(lifetimes);
        format!("{parent}{bridge}::{target}{lifetimes}")
    }

    fn non_terminal_type_src(&self, id: OriginalNonTerminalId, in_non_terminals: bool) -> String {
        let takeoff = self.generator.takeoff;
        let item = takeoff.preflight.non_terminal_defs[id];
        let name = non_terminal_name(item);
        let lifetimes = match item {
            cst::FileItem::Start(_) => unreachable!("Start items are not non-terminals"),
            cst::FileItem::Struct(cst::Struct {
                prefix: _,
                name: _,
                fieldset: _,
            })
            | cst::FileItem::Enum(cst::Enum {
                prefix: _,
                name: _,
                variants: _,
            }) => self.lifetime_list_src(&takeoff.non_terminal_lifetime_sets[id]),
            cst::FileItem::Manual(cst::Manual {
                name: _,
                override_: _,
                lifetimes,
                variants: _,
            }) => declared_lifetime_list_src(lifetimes),
            cst::FileItem::Terminal(_) => unreachable!("Terminal items are not non-terminals"),
            cst::FileItem::Bridge(_) => unreachable!("Bridge items are not non-terminals"),
        };
        let module = if in_non_terminals {
            ""
        } else {
            "non_terminals::"
        };
        format!("{module}{name}{lifetimes}")
    }

    fn lifetime_list_src(&self, lifetimes: &LifetimeSet) -> String {
        generic_list_src(&self.lifetime_names(lifetimes))
    }

    fn lifetime_names(&self, lifetimes: &LifetimeSet) -> Vec<String> {
        lifetimes
            .iter()
            .map(|id| self.generator.takeoff.preflight[id].raw.raw().to_owned())
            .collect()
    }
}

fn non_terminal_name<'a>(item: &cst::FileItem<'a>) -> &'a str {
    match item {
        cst::FileItem::Start(_) => unreachable!("Start items are not non-terminals"),
        cst::FileItem::Struct(cst::Struct {
            prefix: _,
            name,
            fieldset: _,
        }) => name.raw.raw(),
        cst::FileItem::Enum(cst::Enum {
            prefix: _,
            name,
            variants: _,
        }) => name.raw.raw(),
        cst::FileItem::Manual(cst::Manual {
            name,
            override_: _,
            lifetimes: _,
            variants: _,
        }) => name.raw.raw(),
        cst::FileItem::Terminal(_) => unreachable!("Terminal items are not non-terminals"),
        cst::FileItem::Bridge(_) => unreachable!("Bridge items are not non-terminals"),
    }
}

fn tuple_variables(fields: &[cst::TupleField<'_>]) -> Vec<Option<String>> {
    fields
        .iter()
        .enumerate()
        .map(|(i, field)| match field {
            cst::TupleField::Used(_) => Some(format!("t{i}")),
            cst::TupleField::Skipped(_) => None,
        })
        .collect()
}

fn prefix_src(prefix: Option<token::RustStringLiteral<'_>>) -> String {
    let Some(prefix) = prefix else {
        return String::new();
    };
    let raw = prefix.raw.raw();
    let quote = raw
        .find('"')
        .expect("A Rust string literal has an opening quote");
    let hashes = quote - "rust".len();
    let content = &raw[quote + 1..raw.len() - hashes - 1];
    format!("{content}\n")
}

fn declared_lifetime_list_src(lifetimes: &[token::SingleQuoteLowerName<'_>]) -> String {
    generic_list_src(
        &lifetimes
            .iter()
            .map(|name| name.raw.raw().to_owned())
            .collect::<Vec<_>>(),
    )
}

fn generic_list_src(parameters: &[String]) -> String {
    if parameters.is_empty() {
        String::new()
    } else {
        format!("<{}>", parameters.join(", "))
    }
}

fn create_unique_identifier(preferred: &str, used: &mut HashSet<String>) -> String {
    if used.insert(preferred.to_owned()) {
        return preferred.to_owned();
    }
    for suffix in 2.. {
        let name = format!("{preferred}{suffix}");
        if used.insert(name.clone()) {
            return name;
        }
    }
    unreachable!("There are finitely many used identifiers")
}

fn pascal_to_snake_case(name: &str) -> String {
    let mut out = String::new();
    for (i, character) in name.chars().enumerate() {
        if i != 0 && character.is_ascii_uppercase() {
            out.push('_');
        }
        out.push(character.to_ascii_lowercase());
    }
    out
}
