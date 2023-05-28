use crate::data::{ast::*, table::*, validated_file::*, KikiErr, RustSrc};
use std::collections::{HashMap, HashSet};

const NONTERMINAL_DERIVE_CLAUSE: &str = "#[derive(Clone, Debug)]";
const STATE_VARIANT_PREFIX: &str = "S";
const RULE_KIND_VARIANT_PREFIX: &str = "R";
const ACTION_SHIFT_VARIANT_NAME: &str = "Shift";
const ACTION_REDUCE_VARIANT_NAME: &str = "Reduce";
const ACTION_ACCEPT_VARIANT_NAME: &str = "Accept";
const ACTION_ERR_VARIANT_NAME: &str = "Err";

pub fn table_to_rust(table: &Table, file: ValidatedFile) -> Result<RustSrc, KikiErr> {
    let builder = SrcBuilder::new(table, file);
    builder.file_src()
}

#[derive(Debug)]
struct SrcBuilder<'a> {
    table: &'a Table,
    file: ValidatedFile,
    start_type_name: String,
    token_enum_name: String,
    eof_variant_name: String,
    quasitoken_enum_name: String,
    quasitoken_kind_enum_name: String,
    nonterminal_kind_enum_name: String,
    state_enum_name: String,
    node_enum_name: String,
    action_enum_name: String,
    rule_kind_enum_name: String,
    action_table_name: String,
    goto_table_name: String,

    node_to_terminal_method_names: HashMap<DollarlessTerminalName, String>,
}

impl SrcBuilder<'_> {
    fn new(table: &Table, mut file: ValidatedFile) -> SrcBuilder {
        let used_identifiers = &mut file.defined_identifiers;
        let start_type_name = file.start.to_owned();
        let token_enum_name = file.terminal_enum.name.to_owned();
        let eof_variant_name = create_unique_identifier("Eof", used_identifiers);
        let quasitoken_enum_name = create_unique_identifier("Quasitoken", used_identifiers);
        let quasitoken_kind_enum_name =
            create_unique_identifier("QuasitokenKind", used_identifiers);
        let nonterminal_kind_enum_name =
            create_unique_identifier("NonterminalKind", used_identifiers);
        let state_enum_name = create_unique_identifier("State", used_identifiers);
        let node_enum_name = create_unique_identifier("Node", used_identifiers);
        let action_enum_name = create_unique_identifier("Action", used_identifiers);
        let rule_kind_enum_name = create_unique_identifier("RuleKind", used_identifiers);
        let action_table_name = create_unique_identifier("ACTION_TABLE", used_identifiers);
        let goto_table_name = create_unique_identifier("GOTO_TABLE", used_identifiers);

        let node_to_terminal_method_names: HashMap<DollarlessTerminalName, String> = file
            .terminal_enum
            .variants
            .iter()
            .enumerate()
            .map(|(variant_index, variant)| {
                let variant_name_snake_case = pascal_to_snake_case(variant.dollarless_name.raw());
                let variant_name_original_case = variant.dollarless_name.clone();
                let method_name = format!("try_into_{variant_name_snake_case}_{variant_index}");
                (variant_name_original_case, method_name)
            })
            .collect();

        SrcBuilder {
            table,
            file,
            start_type_name,
            token_enum_name,
            eof_variant_name,
            quasitoken_enum_name,
            quasitoken_kind_enum_name,
            nonterminal_kind_enum_name,
            state_enum_name,
            node_enum_name,
            action_enum_name,
            rule_kind_enum_name,
            action_table_name,
            goto_table_name,
            node_to_terminal_method_names,
        }
    }
}

impl SrcBuilder<'_> {
    fn file_src(&self) -> Result<RustSrc, KikiErr> {
        let Self {
            table,
            file,
            start_type_name,
            token_enum_name,
            eof_variant_name,
            quasitoken_enum_name,
            quasitoken_kind_enum_name,
            nonterminal_kind_enum_name,
            state_enum_name,
            node_enum_name,
            action_enum_name,
            rule_kind_enum_name,
            action_table_name,
            goto_table_name,
            ..
        } = self;

        let token_kind_enum_variants_indent_1 = self.get_token_kind_enum_variants_src().indent(1);
        let num_of_token_variants = file.terminal_enum.variants.len();
        let nonterminal_kind_enum_variants_indent_1 =
            self.get_nonterminal_kind_enum_variants_src().indent(1);
        let state_enum_variants_indent_1 = self.get_state_enum_variants_src().indent(1);
        let node_enum_variants_indent_1 = self.get_node_enum_variants_src().indent(1);
        let rule_kind_enum_variants_indent_1 = self.get_rule_kind_enum_variants_src().indent(1);
        let pop_and_reduce_match_arms_indent_2 = self.get_pop_and_reduce_match_arms_src().indent(2);
        let quasitoken_kind_from_token_match_arms_indent_3 = self
            .get_quasitoken_kind_from_token_match_arms_src()
            .indent(3);
        let node_from_token_match_arms_indent_3 =
            self.get_node_from_token_match_arms_src().indent(3);
        let action_table_rows_indent_1 = self.get_action_table_rows_src().indent(1);
        let goto_table_rows_indent_1 = self.get_goto_table_rows_src().indent(1);
        let impl_try_from_node_for_each_nonterminal =
            self.get_impl_try_from_node_for_each_nonterminal_src();
        let node_try_into_terminal_variant_name_variant_index_fns_indent_1 = self
            .get_node_try_into_terminal_variant_name_variant_index_fns_src()
            .indent(1);
        let nonterminal_type_defs = self.get_nonterminal_type_defs_src();

        let num_of_quasitoken_kind_variants = file.terminal_enum.variants.len() + 1;
        let num_of_nonterminal_kind_variants = file.nonterminals.len();
        let num_of_state_variants = table.states();

        Ok(RustSrc(format!(
            r#"// This code was generated by Kiki.

#[derive(Debug)]
enum {quasitoken_enum_name} {{
    Token({token_enum_name}),
    {eof_variant_name},
}}

#[derive(Clone, Copy, Debug)]
enum {quasitoken_kind_enum_name} {{
{token_kind_enum_variants_indent_1}
    {eof_variant_name} = {num_of_token_variants},
}}

#[derive(Clone, Copy, Debug)]
enum {nonterminal_kind_enum_name} {{
{nonterminal_kind_enum_variants_indent_1}
}}

#[derive(Clone, Copy, Debug)]
enum {state_enum_name} {{
{state_enum_variants_indent_1}
}}

#[derive(Debug)]
enum {node_enum_name} {{
{node_enum_variants_indent_1}
}}

#[derive(Clone, Copy, Debug)]
enum {action_enum_name} {{
    {ACTION_SHIFT_VARIANT_NAME}({state_enum_name}),
    {ACTION_REDUCE_VARIANT_NAME}({rule_kind_enum_name}),
    {ACTION_ACCEPT_VARIANT_NAME},
    {ACTION_ERR_VARIANT_NAME},
}}

#[derive(Clone, Copy, Debug)]
enum {rule_kind_enum_name} {{
{rule_kind_enum_variants_indent_1}
}}

/// If the parser encounters an unexpected token `t`, it will return `Err(Some(t))`.
/// If the parser encounters an unexpected end of input, it will return `Err(None)`.
pub fn parse<S>(src: S) -> Result<{start_type_name}, Option<{token_enum_name}>>
where S: IntoIterator<Item = {token_enum_name}> {{
    let mut tokens = src.into_iter()
        .map({quasitoken_enum_name}::Token)
        .chain(std::iter::once({quasitoken_enum_name}::Eof))
        .peekable();
    let mut states = vec![{state_enum_name}::{STATE_VARIANT_PREFIX}0];
    let mut nodes: Vec<{node_enum_name}> = vec![];
    loop {{
        let top_state = *states.last().unwrap();
        let next_quasitoken_kind = {quasitoken_kind_enum_name}::from_quasitoken(tokens.peek().unwrap());
        match get_action(top_state, next_quasitoken_kind) {{
            {action_enum_name}::{ACTION_SHIFT_VARIANT_NAME}(new_state) => {{
                states.push(new_state);
                nodes.push({node_enum_name}::from_token(tokens.next().unwrap().try_into_token().unwrap()));
            }}

            {action_enum_name}::{ACTION_REDUCE_VARIANT_NAME}(rule_kind) => {{
                let (new_node, new_node_kind) = pop_and_reduce(&mut states, &mut nodes, rule_kind);
                nodes.push(new_node);
                let temp_top_state = *states.last().unwrap();
                let Some(new_state) = get_goto(temp_top_state, new_node_kind) else {{
                    return Err(tokens.next().unwrap().try_into_token().ok());
                }};
                states.push(new_state);
            }}

            {action_enum_name}::{ACTION_ACCEPT_VARIANT_NAME} => {{
                return Ok({start_type_name}::try_from(nodes.pop().unwrap()).unwrap());
            }}

            {action_enum_name}::{ACTION_ERR_VARIANT_NAME} => {{
                return Err(tokens.next().unwrap().try_into_token().ok());
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
            {quasitoken_enum_name}::Token(token) => Self::from_token(token),
            {quasitoken_enum_name}::{eof_variant_name} => Self::{eof_variant_name},
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

impl {quasitoken_enum_name} {{
    fn try_into_token(self) -> Result<{token_enum_name}, ()> {{
        match self {{
            Self::Token(token) => Ok(token),
            Self::{eof_variant_name} => Err(()),
        }}
    }}
}}

const {action_table_name}: [[{action_enum_name}; {num_of_quasitoken_kind_variants}]; {num_of_state_variants}] = [
{action_table_rows_indent_1}
];

fn get_action(top_state: {state_enum_name}, next_quasitoken_kind: {quasitoken_kind_enum_name}) -> {action_enum_name} {{
    {action_table_name}[top_state as usize][next_quasitoken_kind as usize]
}}

const {goto_table_name}: [[Option<{state_enum_name}>; {num_of_nonterminal_kind_variants}]; {num_of_state_variants}] = [
{goto_table_rows_indent_1}
];

fn get_goto(top_state: {state_enum_name}, new_node_kind: {nonterminal_kind_enum_name}) -> Option<{state_enum_name}> {{
    {goto_table_name}[top_state as usize][new_node_kind as usize]
}}

{impl_try_from_node_for_each_nonterminal}

impl {node_enum_name} {{
{node_try_into_terminal_variant_name_variant_index_fns_indent_1}
}}

{nonterminal_type_defs}
"#
        )))
    }

    fn get_token_kind_enum_variants_src(&self) -> String {
        self.file
            .terminal_enum
            .variants
            .iter()
            .enumerate()
            .map(|(variant_index, variant)| {
                let name = variant.dollarless_name.raw();
                format!("{name} = {variant_index},")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn get_nonterminal_kind_enum_variants_src(&self) -> String {
        self.file
            .nonterminals
            .iter()
            .enumerate()
            .map(|(nonterminal_index, nonterminal)| {
                let nonterminal_name = nonterminal.name();
                format!("{nonterminal_name} = {nonterminal_index},")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn get_state_enum_variants_src(&self) -> String {
        (0..self.table.states())
            .map(|i| format!("{STATE_VARIANT_PREFIX}{i} = {i},"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn get_node_enum_variants_src(&self) -> String {
        self.file
            .nonterminals
            .iter()
            .map(|nonterminal| format!("{name}({name}),", name = nonterminal.name()))
            .chain(self.file.terminal_enum.variants.iter().map(|variant| {
                let name = variant.dollarless_name.raw();
                let type_ = &variant.type_;
                format!("{name}({type_}),")
            }))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn get_rule_kind_enum_variants_src(&self) -> String {
        (0..self.get_number_of_rule_kinds())
            .map(|i| format!("{RULE_KIND_VARIANT_PREFIX}{i} = {i},"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn get_number_of_rule_kinds(&self) -> usize {
        self.file
            .nonterminals
            .iter()
            .map(|nonterminal| match nonterminal {
                Nonterminal::Struct(_) => 1,
                Nonterminal::Enum(e) => e.variants.len(),
            })
            .sum()
    }

    fn get_pop_and_reduce_match_arms_src(&self) -> String {
        self.get_rules()
            .enumerate()
            .map(|(rule_index, (constructor_name, fieldset))| {
                self.get_rule_reduction_src(rule_index, constructor_name, fieldset)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn get_rules(&self) -> impl Iterator<Item = (ConstructorName<'_>, &Fieldset)> {
        self.file
            .nonterminals
            .iter()
            .flat_map(|nonterminal| match nonterminal {
                Nonterminal::Struct(s) => {
                    vec![(ConstructorName::Struct(&s.name.name), &s.fieldset)]
                }
                Nonterminal::Enum(e) => e
                    .variants
                    .iter()
                    .map(|v| {
                        let enum_name = &e.name.name;
                        let variant_name = &v.name.name;
                        (
                            ConstructorName::EnumVariant {
                                enum_name,
                                variant_name,
                            },
                            &v.fieldset,
                        )
                    })
                    .collect(),
            })
    }

    fn get_rule_reduction_src(
        &self,
        rule_index: usize,
        constructor_name: ConstructorName,
        fieldset: &Fieldset,
    ) -> String {
        let reduction_code_indent_1: String = match fieldset {
            Fieldset::Empty => self.get_empty_fieldset_rule_reduction_src(constructor_name),
            Fieldset::Named(NamedFieldset { fields }) => {
                self.get_named_fieldset_rule_reduction_src(constructor_name, fields)
            }
            Fieldset::Tuple(TupleFieldset { fields }) => {
                self.get_tuple_fieldset_rule_reduction_src(constructor_name, fields)
            }
        }
        .indent(1);
        let rule_kind_enum_name = &self.rule_kind_enum_name;
        format!(
            r#"{rule_kind_enum_name}::{RULE_KIND_VARIANT_PREFIX}{rule_index} => {{
{reduction_code_indent_1}
}}"#
        )
    }

    fn get_empty_fieldset_rule_reduction_src(&self, constructor_name: ConstructorName) -> String {
        // TODO
        constructor_name.to_string()
    }

    fn get_named_fieldset_rule_reduction_src(
        &self,
        constructor_name: ConstructorName,
        fields: &[NamedField],
    ) -> String {
        let node_enum_name = &self.node_enum_name;
        let nonterminal_kind_enum_name = &self.nonterminal_kind_enum_name;
        let parent_type_name = constructor_name.type_name();
        let constructor_name = constructor_name.to_string();
        let child_vars: String = fields
            .iter()
            .enumerate()
            .rev()
            .map(|(field_index, field)| match (&field.name, &field.symbol) {
                (IdentOrUnderscore::Underscore, _) => "nodes.pop().unwrap();\n".to_owned(),
                (IdentOrUnderscore::Ident(field_name), IdentOrTerminalIdent::Ident(field_type)) => {
                    let field_name = &field_name.name;
                    let field_type_name = &field_type.name;
                    format!("let {field_name}_{field_index} = {field_type_name}::try_from(nodes.pop().unwrap()).unwrap();\n")
                },
                (IdentOrUnderscore::Ident(field_name), IdentOrTerminalIdent::Terminal(field_type)) => {
                    let field_name = &field_name.name;
                    let try_into_method_name = self.node_to_terminal_method_names.get(&field_type.dollarless_name()).unwrap();
                    format!("let {field_name}_{field_index} = nodes.pop().unwrap().{try_into_method_name}().unwrap();\n")
                }
            })
            .collect();

        let parent_fields_indent_2: String = fields
            .iter()
            .enumerate()
            .filter_map(|(field_index, field)| match &field.name {
                IdentOrUnderscore::Underscore => None,
                IdentOrUnderscore::Ident(field_name) => {
                    let field_name = &field_name.name;
                    Some(format!("{field_name}: {field_name}_{field_index},"))
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
            .indent(2);

        format!(
            r#"(
    {node_enum_name}::{parent_type_name}({child_vars}{constructor_name}(
{parent_fields_indent_2}
    )),
    {nonterminal_kind_enum_name}::{parent_type_name}
)"#
        )
    }

    fn get_tuple_fieldset_rule_reduction_src(
        &self,
        constructor_name: ConstructorName,
        fields: &[TupleField],
    ) -> String {
        const ANONYMOUS_FIELD_PREFIX: &str = "t";
        let constructor_name = constructor_name.to_string();
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
                    let try_into_method_name = self.node_to_terminal_method_names.get(&field_type.dollarless_name()).unwrap();
                    format!("let {ANONYMOUS_FIELD_PREFIX}{field_index} = nodes.pop().unwrap().{try_into_method_name}().unwrap();\n")
                },
            })
            .collect();

        let parent_fields_indent_1: String = fields
            .iter()
            .enumerate()
            .filter_map(|(field_index, field)| match field {
                TupleField::Skipped(_) => None,
                TupleField::Used(_) => Some(format!("{ANONYMOUS_FIELD_PREFIX}{field_index},")),
            })
            .collect::<Vec<_>>()
            .join("\n")
            .indent(1);

        format!("{child_vars}{constructor_name}(\n{parent_fields_indent_1}\n)")
    }

    fn get_quasitoken_kind_from_token_match_arms_src(&self) -> String {
        let token_enum_name = &self.token_enum_name;
        self.file
            .terminal_enum
            .variants
            .iter()
            .map(|variant| {
                let name = variant.dollarless_name.raw();
                format!("{token_enum_name}::{name}(_) => Self::{name},")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn get_node_from_token_match_arms_src(&self) -> String {
        let token_enum_name = &self.token_enum_name;
        self.file
            .terminal_enum
            .variants
            .iter()
            .map(|variant| {
                let name = variant.dollarless_name.raw();
                format!("{token_enum_name}::{name}(t) => Self::{name}(t),")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn get_action_table_rows_src(&self) -> String {
        (0..self.table.states())
            .map(|state| self.get_action_table_row_src(state))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn get_action_table_row_src(&self, state: usize) -> String {
        let action_enum_name = &self.action_enum_name;
        let row_items_indent_1 = self
            .table
            .dollarless_terminals
            .iter()
            .map(Quasiterminal::Terminal)
            .chain(std::iter::once(Quasiterminal::Eof))
            .map(|quasiterminal| {
                let action = self.table.action(state, quasiterminal);
                let unqualified_variant = self.get_action_variant_unqualified_src(action);
                format!("{action_enum_name}::{unqualified_variant},")
            })
            .collect::<Vec<_>>()
            .join("\n")
            .indent(1);
        format!("[\n{row_items_indent_1}\n],")
    }

    fn get_action_variant_unqualified_src(&self, action: Action) -> String {
        let state_enum_name = &self.state_enum_name;
        let rule_kind_enum_name = &self.rule_kind_enum_name;
        match action {
            Action::Shift(state_index) => {
                format!("{ACTION_SHIFT_VARIANT_NAME}({state_enum_name}::{STATE_VARIANT_PREFIX}{state_index})")
            }
            Action::Reduce(rule_index) => {
                format!("{ACTION_REDUCE_VARIANT_NAME}({rule_kind_enum_name}::{RULE_KIND_VARIANT_PREFIX}{rule_index})")
            }
            Action::Accept => format!("{ACTION_ACCEPT_VARIANT_NAME}"),
            Action::Err => format!("{ACTION_ERR_VARIANT_NAME}"),
        }
    }

    fn get_goto_table_rows_src(&self) -> String {
        (0..self.table.states())
            .map(|state| self.get_goto_table_row_src(state))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn get_goto_table_row_src(&self, state: usize) -> String {
        let row_items_indent_1 = self
            .table
            .nonterminals
            .iter()
            .map(|nonterminal| {
                let goto = self.table.goto(state, nonterminal);
                let qualified_variant = self.get_goto_variant_qualified_src(goto);
                format!("{qualified_variant},")
            })
            .collect::<Vec<_>>()
            .join("\n")
            .indent(1);
        format!("[\n{row_items_indent_1}\n],")
    }

    fn get_goto_variant_qualified_src(&self, goto: Goto) -> String {
        let state_enum_name = &self.state_enum_name;
        match goto {
            Goto::State(state_index) => {
                format!("Some({state_enum_name}::{STATE_VARIANT_PREFIX}{state_index})")
            }
            Goto::Err => format!("None"),
        }
    }

    fn get_impl_try_from_node_for_each_nonterminal_src(&self) -> String {
        let node_enum_name = &self.node_enum_name;
        self.file
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
            .join("\n\n")
    }

    fn get_node_try_into_terminal_variant_name_variant_index_fns_src(&self) -> String {
        self.file
            .terminal_enum
            .variants
            .iter()
            .map(|variant| {
                let variant_name_original_case = variant.dollarless_name.raw();
                let method_name = self
                    .node_to_terminal_method_names
                    .get(&variant.dollarless_name)
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
    }

    fn get_nonterminal_type_defs_src(&self) -> String {
        self.file.nonterminals
            .iter()
            .map(|nonterminal| match nonterminal {
                Nonterminal::Struct(s) => {
                    let nonterminal_name = &s.name.name;
                    let fieldset = self.get_fieldset_src_with_semicolon_if_unnamed(&s.fieldset);
                    format!("{NONTERMINAL_DERIVE_CLAUSE}\npub struct {nonterminal_name}{fieldset}")
                },
                Nonterminal::Enum(e) => {
                    let nonterminal_name = &e.name.name;
                    let variants_indent_1 = e.variants
                        .iter()
                        .map(|variant| {
                            let variant_name = &variant.name.name;
                            let variant_fieldset = self.get_fieldset_src_without_semicolon(&variant.fieldset);
                            format!("{variant_name}{variant_fieldset},")
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                        .indent(1);
                    format!("{NONTERMINAL_DERIVE_CLAUSE}\npub enum {nonterminal_name} {{\n{variants_indent_1}\n}}")
                },
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    fn get_fieldset_src_with_semicolon_if_unnamed(&self, fieldset: &Fieldset) -> String {
        self.get_fieldset_src(fieldset, true)
    }

    fn get_fieldset_src_without_semicolon(&self, fieldset: &Fieldset) -> String {
        self.get_fieldset_src(fieldset, false)
    }

    fn get_fieldset_src(&self, fieldset: &Fieldset, use_semicolon_if_unnamed: bool) -> String {
        match fieldset {
            Fieldset::Empty => self.get_empty_fieldset_src(use_semicolon_if_unnamed),
            Fieldset::Named(fieldset) => self.get_named_fieldset_src(fieldset),
            Fieldset::Tuple(fieldset) => {
                self.get_tuple_fieldset_src(fieldset, use_semicolon_if_unnamed)
            }
        }
    }

    fn get_empty_fieldset_src(&self, use_semicolon: bool) -> String {
        if use_semicolon { ";" } else { "" }.to_owned()
    }

    fn get_named_fieldset_src(&self, fieldset: &NamedFieldset) -> String {
        let fields_indent_1 = fieldset
            .fields
            .iter()
            .filter_map(|field| match (&field.name, &field.symbol) {
                (IdentOrUnderscore::Underscore, _) => None,
                (IdentOrUnderscore::Ident(field_name), IdentOrTerminalIdent::Ident(field_type)) => {
                    let field_name = &field_name.name;
                    let field_type_name = &field_type.name;
                    Some(format!("{field_name}: {field_type_name},"))
                }
                (
                    IdentOrUnderscore::Ident(field_name),
                    IdentOrTerminalIdent::Terminal(field_type),
                ) => {
                    let field_name = &field_name.name;
                    let field_type_name = self
                        .file
                        .terminal_enum
                        .get_type(&field_type.dollarless_name())
                        .unwrap();
                    Some(format!("{field_name}: {field_type_name},"))
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
            .indent(1);
        format!(" {{\n{fields_indent_1}\n}}")
    }

    fn get_tuple_fieldset_src(&self, fieldset: &TupleFieldset, use_semicolon: bool) -> String {
        let fields_indent_1 = fieldset
            .fields
            .iter()
            .filter_map(|field| match field {
                TupleField::Skipped(_) => None,
                TupleField::Used(IdentOrTerminalIdent::Ident(field_type)) => {
                    let field_type_name = &field_type.name;
                    Some(format!("{field_type_name},"))
                }
                TupleField::Used(IdentOrTerminalIdent::Terminal(field_type)) => {
                    let field_type_name = self
                        .file
                        .terminal_enum
                        .get_type(&field_type.dollarless_name())
                        .unwrap();
                    Some(format!("{field_type_name},"))
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
            .indent(1);
        let possible_semicolon = if use_semicolon { ";" } else { "" };
        format!("(\n{fields_indent_1}\n){possible_semicolon}")
    }
}

#[derive(Debug, Clone, Copy)]
enum ConstructorName<'a> {
    Struct(&'a str),
    EnumVariant {
        enum_name: &'a str,
        variant_name: &'a str,
    },
}

impl ConstructorName<'_> {
    fn to_string(&self) -> String {
        match self {
            ConstructorName::Struct(name) => name.to_string(),
            ConstructorName::EnumVariant {
                enum_name,
                variant_name,
            } => format!("{enum_name}::{variant_name}"),
        }
    }

    fn type_name(&self) -> &str {
        match self {
            ConstructorName::Struct(name) => name,
            ConstructorName::EnumVariant { enum_name, .. } => enum_name,
        }
    }
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

    if let Some(c) = chars.next() {
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

    use crate::data::{validated_file::TerminalVariant, ByteIndex};

    #[test]
    fn balanced_parens() {
        let actions = {
            use Action::*;
            [
                [Shift(2), Err, Reduce(1)],
                [Err, Err, Accept],
                [Shift(2), Reduce(1), Err],
                [Err, Shift(4), Err],
                [Err, Reduce(2), Reduce(2)],
            ]
            .into_iter()
            .flatten()
            .collect()
        };
        let gotos = {
            use Goto::*;
            vec![State(1), Err, State(3), Err, Err]
        };
        let table = Table {
            dollarless_terminals: vec![
                DollarlessTerminalName::remove_dollars("LParen"),
                DollarlessTerminalName::remove_dollars("RParen"),
            ],
            nonterminals: vec!["Expr".to_string()],
            actions,
            gotos,
        };
        let file = ValidatedFile {
            start: "Expr".to_owned(),
            terminal_enum: TerminalEnum {
                name: "Token".to_string(),
                variants: vec![
                    TerminalVariant {
                        dollarless_name: DollarlessTerminalName::remove_dollars("LParen"),
                        type_: "()".to_string(),
                    },
                    TerminalVariant {
                        dollarless_name: DollarlessTerminalName::remove_dollars("RParen"),
                        type_: "()".to_string(),
                    },
                ],
            },
            nonterminals: vec![Nonterminal::Enum(Enum {
                name: positionless_ident("Expr"),
                variants: vec![
                    EnumVariant {
                        name: positionless_ident("Empty"),
                        fieldset: Fieldset::Empty,
                    },
                    EnumVariant {
                        name: positionless_ident("Wrap"),
                        fieldset: Fieldset::Tuple(TupleFieldset {
                            fields: vec![
                                TupleField::Used(IdentOrTerminalIdent::Terminal(
                                    positionless_terminal_ident(
                                        &DollarlessTerminalName::remove_dollars("LParen"),
                                    ),
                                )),
                                TupleField::Used(IdentOrTerminalIdent::Ident(positionless_ident(
                                    "Expr",
                                ))),
                                TupleField::Used(IdentOrTerminalIdent::Terminal(
                                    positionless_terminal_ident(
                                        &DollarlessTerminalName::remove_dollars("RParen"),
                                    ),
                                )),
                            ],
                        }),
                    },
                ],
            })],
            defined_identifiers: vec!["Expr", "Token", "LParen", "RParen"]
                .into_iter()
                .map(ToOwned::to_owned)
                .collect(),
        };

        let RustSrc(rust_src) = table_to_rust(&table, file).unwrap();
        insta::assert_debug_snapshot!(rust_src);
    }

    fn positionless_ident(s: &str) -> Ident {
        Ident {
            name: s.to_owned(),
            position: ByteIndex(0),
        }
    }

    fn positionless_terminal_ident(s: &DollarlessTerminalName) -> TerminalIdent {
        TerminalIdent {
            dollared_name: format!("${}", s.raw()),
            position: ByteIndex(0),
        }
    }

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
