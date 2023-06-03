use super::*;

pub(super) fn get_first_sets(rules: &[Rule]) -> HashMap<String, FirstSet> {
    let builder = FirstSetMapBuilder { rules };
    builder.get_first_sets()
}

struct FirstSetMapBuilder<'a> {
    rules: &'a [Rule<'a>],
}

impl FirstSetMapBuilder<'_> {
    fn get_first_sets(self) -> HashMap<String, FirstSet> {
        let mut out = self.get_a_map_of_each_nonterminal_to_the_empty_set();

        loop {
            let DidChange(changed) = self.expand(&mut out);
            if !changed {
                return out;
            }
        }
    }

    fn get_a_map_of_each_nonterminal_to_the_empty_set(&self) -> HashMap<String, FirstSet> {
        let mut out = HashMap::new();
        for name in self.get_nonterminal_names() {
            out.insert(
                name.to_owned(),
                FirstSet {
                    terminals: Oset::new(),
                    contains_epsilon: false,
                },
            );
        }
        out
    }

    fn get_nonterminal_names(&self) -> Oset<&str> {
        self.rules
            .iter()
            .map(|rule| rule.constructor_name.type_name())
            .collect()
    }

    fn expand(&self, out: &mut HashMap<String, FirstSet>) -> DidChange {
        let mut changed = DidChange(false);
        for rule in self.rules {
            changed |= expand_rule(rule, out);
        }
        changed
    }
}

fn expand_rule(rule: &Rule, out: &mut HashMap<String, FirstSet>) -> DidChange {
    let current_first = get_current_first_set(&rule.fieldset, out);
    let first_set = out.get_mut(rule.constructor_name.type_name()).unwrap();
    add_all(current_first, first_set)
}

fn get_current_first_set(fieldset: &Fieldset, map: &HashMap<String, FirstSet>) -> FirstSet {
    match fieldset {
        Fieldset::Empty => get_current_first_set_for_empty_fieldset(),
        Fieldset::Named(named) => get_current_first_set_for_named_fieldset(named, map),
        Fieldset::Tuple(tuple) => get_current_first_set_for_tuple_fieldset(tuple, map),
    }
}

fn get_current_first_set_for_named_fieldset(
    named: &NamedFieldset,
    map: &HashMap<String, FirstSet>,
) -> FirstSet {
    let mut out = FirstSet {
        terminals: Oset::new(),
        contains_epsilon: true,
    };

    for field in &named.fields {
        let first = get_current_first_set_for_symbol(&field.symbol, map);
        out.terminals.extend(first.terminals);

        if !first.contains_epsilon {
            out.contains_epsilon = false;
            break;
        }
    }

    out
}

fn get_current_first_set_for_tuple_fieldset(
    tuple: &TupleFieldset,
    map: &HashMap<String, FirstSet>,
) -> FirstSet {
    let mut out = FirstSet {
        terminals: Oset::new(),
        contains_epsilon: true,
    };

    for field in &tuple.fields {
        let first = get_current_first_set_for_symbol(field.symbol(), map);
        out.terminals.extend(first.terminals);

        if !first.contains_epsilon {
            out.contains_epsilon = false;
            break;
        }
    }

    out
}

fn get_current_first_set_for_symbol(
    symbol: &IdentOrTerminalIdent,
    map: &HashMap<String, FirstSet>,
) -> FirstSet {
    match symbol {
        IdentOrTerminalIdent::Ident(ident) => map.get(&ident.name).unwrap().clone(),
        IdentOrTerminalIdent::Terminal(terminal_ident) => FirstSet {
            terminals: [terminal_ident.name.clone()].into_iter().collect(),
            contains_epsilon: false,
        },
    }
}

fn get_current_first_set_for_empty_fieldset() -> FirstSet {
    FirstSet {
        terminals: Oset::new(),
        contains_epsilon: true,
    }
}

fn add_all(new: FirstSet, out: &mut FirstSet) -> DidChange {
    let old_len = out.terminals.len();
    let did_contain_epsilon = out.contains_epsilon;

    out.terminals.extend(new.terminals);
    out.contains_epsilon |= new.contains_epsilon;

    DidChange(out.terminals.len() != old_len || out.contains_epsilon != did_contain_epsilon)
}

#[derive(Debug, Clone, Copy)]
struct DidChange(bool);

impl std::ops::BitOrAssign for DidChange {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
