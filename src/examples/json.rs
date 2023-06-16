// This code was generated by Kiki.
// Kiki is an open-source minimalist parser generator for Rust.
// You can read more at https://crates.io/crates/kiki
//
// This code was generated from a grammar with the following hash:
// @sha256 0887494eb5b928d9ebb0c4ae3b2475b5cdbea535d17e78bc7c0b66d06a4851c3

// Since this code is automatically generated,
// some parts may be unidiomatic.
// The linter often complains about these parts.
// However, these warnings are not useful.
// Therefore, we disable certain lints for this file.
#![allow(non_snake_case)]
#![allow(dead_code)]

#[derive(Debug)]
pub enum Token {
    String(String),
    Num(String),
    Bool(String),
    LCurly(String),
    RCurly(String),
    LSquare(String),
    RSquare(String),
    Colon(String),
    Comma(String),
}

#[derive(Clone, Debug)]
pub enum Json {
    Obj(
        Box<Obj>,
    ),
    Arr(
        Box<Arr>,
    ),
}

#[derive(Clone, Debug)]
pub struct Obj {
    pub entries: Box<OptEntries>,
}

#[derive(Clone, Debug)]
pub enum OptEntries {
    None,
    Some(
        Box<Entries>,
    ),
}

#[derive(Clone, Debug)]
pub enum Entries {
    One(
        Box<Entry>,
    ),
    Many(
        Box<Entries>,
        Box<Entry>,
    ),
}

#[derive(Clone, Debug)]
pub struct Entry {
    pub key: String,
    pub val: Box<Expr>,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Obj(
        Box<Obj>,
    ),
    Arr(
        Box<Arr>,
    ),
    String(
        String,
    ),
    Num(
        String,
    ),
    Bool(
        String,
    ),
}

#[derive(Clone, Debug)]
pub struct Arr {
    pub elements: Box<OptElements>,
}

#[derive(Clone, Debug)]
pub enum OptElements {
    None,
    Some(
        Box<Elements>,
    ),
}

#[derive(Clone, Debug)]
pub enum Elements {
    One(
        Box<Expr>,
    ),
    Many(
        Box<Elements>,
        Box<Expr>,
    ),
}

/// If the parser encounters an unexpected token `t`, it will return `Err(Some(t))`.
/// If the parser encounters an unexpected end of input, it will return `Err(None)`.
pub fn parse<S>(src: S) -> Result<Json, Option<Token>>
where S: IntoIterator<Item = Token> {
    let mut quasiterminals = src.into_iter()
        .map(Quasiterminal::Terminal)
        .chain(std::iter::once(Quasiterminal::Eof))
        .peekable();
    let mut states = vec![State::S0];
    let mut nodes: Vec<Node> = vec![];
    loop {
        let top_state = *states.last().unwrap();
        let next_quasiterminal_kind = QuasiterminalKind::from_quasiterminal(quasiterminals.peek().unwrap());
        match get_action(top_state, next_quasiterminal_kind) {
            Action::Shift(new_state) => {
                states.push(new_state);
                nodes.push(Node::from_terminal(quasiterminals.next().unwrap().try_into_terminal().unwrap()));
            }

            Action::Reduce(rule_kind) => {
                let (new_node, new_node_kind) = pop_and_reduce(&mut states, &mut nodes, rule_kind);
                nodes.push(new_node);
                let temp_top_state = *states.last().unwrap();
                let Some(new_state) = get_goto(temp_top_state, new_node_kind) else {
                    return Err(quasiterminals.next().unwrap().try_into_terminal().ok());
                };
                states.push(new_state);
            }

            Action::Accept => {
                return Ok(Json::try_from(nodes.pop().unwrap()).unwrap());
            }

            Action::Err => {
                return Err(quasiterminals.next().unwrap().try_into_terminal().ok());
            }
        }
    }
}

#[derive(Debug)]
enum Quasiterminal {
    Terminal(Token),
    Eof,
}

#[derive(Clone, Copy, Debug)]
enum QuasiterminalKind {
    String = 0,
    Num = 1,
    Bool = 2,
    LCurly = 3,
    RCurly = 4,
    LSquare = 5,
    RSquare = 6,
    Colon = 7,
    Comma = 8,
    Eof = 9,
}

#[derive(Clone, Copy, Debug)]
enum NonterminalKind {
    Json = 0,
    Obj = 1,
    OptEntries = 2,
    Entries = 3,
    Entry = 4,
    Expr = 5,
    Arr = 6,
    OptElements = 7,
    Elements = 8,
}

#[derive(Clone, Copy, Debug)]
enum State {
    S0 = 0,
    S1 = 1,
    S2 = 2,
    S3 = 3,
    S4 = 4,
    S5 = 5,
    S6 = 6,
    S7 = 7,
    S8 = 8,
    S9 = 9,
    S10 = 10,
    S11 = 11,
    S12 = 12,
    S13 = 13,
    S14 = 14,
    S15 = 15,
    S16 = 16,
    S17 = 17,
    S18 = 18,
    S19 = 19,
    S20 = 20,
    S21 = 21,
    S22 = 22,
    S23 = 23,
    S24 = 24,
    S25 = 25,
}

#[derive(Debug)]
enum Node {
    Json(Json),
    Obj(Obj),
    OptEntries(OptEntries),
    Entries(Entries),
    Entry(Entry),
    Expr(Expr),
    Arr(Arr),
    OptElements(OptElements),
    Elements(Elements),
    String(String),
    Num(String),
    Bool(String),
    LCurly(String),
    RCurly(String),
    LSquare(String),
    RSquare(String),
    Colon(String),
    Comma(String),
}

#[derive(Clone, Copy, Debug)]
enum Action {
    Shift(State),
    Reduce(RuleKind),
    Accept,
    Err,
}

#[derive(Clone, Copy, Debug)]
enum RuleKind {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15,
    R16 = 16,
    R17 = 17,
}

fn pop_and_reduce(states: &mut Vec<State>, nodes: &mut Vec<Node>, rule_kind: RuleKind) -> (Node, NonterminalKind) {
    match rule_kind {
        RuleKind::R0 => {
            let t0 = Box::new(Obj::try_from(nodes.pop().unwrap()).unwrap());
            
            states.truncate(states.len() - 1);
            
            (
                Node::Json(Json::Obj(
                    t0,
                )),
                NonterminalKind::Json,
            )
        }
        RuleKind::R1 => {
            let t0 = Box::new(Arr::try_from(nodes.pop().unwrap()).unwrap());
            
            states.truncate(states.len() - 1);
            
            (
                Node::Json(Json::Arr(
                    t0,
                )),
                NonterminalKind::Json,
            )
        }
        RuleKind::R2 => {
            nodes.pop().unwrap();
            let entries_1 = Box::new(OptEntries::try_from(nodes.pop().unwrap()).unwrap());
            nodes.pop().unwrap();
            
            states.truncate(states.len() - 3);
            
            (
                Node::Obj(Obj {
                    entries: entries_1,
                }),
                NonterminalKind::Obj,
            )
        }
        RuleKind::R3 => {
            (
                Node::OptEntries(OptEntries::None),
                NonterminalKind::OptEntries,
            )
        }
        RuleKind::R4 => {
            let t0 = Box::new(Entries::try_from(nodes.pop().unwrap()).unwrap());
            
            states.truncate(states.len() - 1);
            
            (
                Node::OptEntries(OptEntries::Some(
                    t0,
                )),
                NonterminalKind::OptEntries,
            )
        }
        RuleKind::R5 => {
            let t0 = Box::new(Entry::try_from(nodes.pop().unwrap()).unwrap());
            
            states.truncate(states.len() - 1);
            
            (
                Node::Entries(Entries::One(
                    t0,
                )),
                NonterminalKind::Entries,
            )
        }
        RuleKind::R6 => {
            let t2 = Box::new(Entry::try_from(nodes.pop().unwrap()).unwrap());
            nodes.pop().unwrap();
            let t0 = Box::new(Entries::try_from(nodes.pop().unwrap()).unwrap());
            
            states.truncate(states.len() - 3);
            
            (
                Node::Entries(Entries::Many(
                    t0,
                    t2,
                )),
                NonterminalKind::Entries,
            )
        }
        RuleKind::R7 => {
            let val_2 = Box::new(Expr::try_from(nodes.pop().unwrap()).unwrap());
            nodes.pop().unwrap();
            let key_0 = nodes.pop().unwrap().try_into_string_0().unwrap();
            
            states.truncate(states.len() - 3);
            
            (
                Node::Entry(Entry {
                    key: key_0,
                    val: val_2,
                }),
                NonterminalKind::Entry,
            )
        }
        RuleKind::R8 => {
            let t0 = Box::new(Obj::try_from(nodes.pop().unwrap()).unwrap());
            
            states.truncate(states.len() - 1);
            
            (
                Node::Expr(Expr::Obj(
                    t0,
                )),
                NonterminalKind::Expr,
            )
        }
        RuleKind::R9 => {
            let t0 = Box::new(Arr::try_from(nodes.pop().unwrap()).unwrap());
            
            states.truncate(states.len() - 1);
            
            (
                Node::Expr(Expr::Arr(
                    t0,
                )),
                NonterminalKind::Expr,
            )
        }
        RuleKind::R10 => {
            let t0 = nodes.pop().unwrap().try_into_string_0().unwrap();
            
            states.truncate(states.len() - 1);
            
            (
                Node::Expr(Expr::String(
                    t0,
                )),
                NonterminalKind::Expr,
            )
        }
        RuleKind::R11 => {
            let t0 = nodes.pop().unwrap().try_into_num_1().unwrap();
            
            states.truncate(states.len() - 1);
            
            (
                Node::Expr(Expr::Num(
                    t0,
                )),
                NonterminalKind::Expr,
            )
        }
        RuleKind::R12 => {
            let t0 = nodes.pop().unwrap().try_into_bool_2().unwrap();
            
            states.truncate(states.len() - 1);
            
            (
                Node::Expr(Expr::Bool(
                    t0,
                )),
                NonterminalKind::Expr,
            )
        }
        RuleKind::R13 => {
            nodes.pop().unwrap();
            let elements_1 = Box::new(OptElements::try_from(nodes.pop().unwrap()).unwrap());
            nodes.pop().unwrap();
            
            states.truncate(states.len() - 3);
            
            (
                Node::Arr(Arr {
                    elements: elements_1,
                }),
                NonterminalKind::Arr,
            )
        }
        RuleKind::R14 => {
            (
                Node::OptElements(OptElements::None),
                NonterminalKind::OptElements,
            )
        }
        RuleKind::R15 => {
            let t0 = Box::new(Elements::try_from(nodes.pop().unwrap()).unwrap());
            
            states.truncate(states.len() - 1);
            
            (
                Node::OptElements(OptElements::Some(
                    t0,
                )),
                NonterminalKind::OptElements,
            )
        }
        RuleKind::R16 => {
            let t0 = Box::new(Expr::try_from(nodes.pop().unwrap()).unwrap());
            
            states.truncate(states.len() - 1);
            
            (
                Node::Elements(Elements::One(
                    t0,
                )),
                NonterminalKind::Elements,
            )
        }
        RuleKind::R17 => {
            let t2 = Box::new(Expr::try_from(nodes.pop().unwrap()).unwrap());
            nodes.pop().unwrap();
            let t0 = Box::new(Elements::try_from(nodes.pop().unwrap()).unwrap());
            
            states.truncate(states.len() - 3);
            
            (
                Node::Elements(Elements::Many(
                    t0,
                    t2,
                )),
                NonterminalKind::Elements,
            )
        }
    }
}

impl QuasiterminalKind {
    fn from_quasiterminal(quasiterminal: &Quasiterminal) -> Self {
        match quasiterminal {
            Quasiterminal::Terminal(terminal) => Self::from_terminal(terminal),
            Quasiterminal::Eof => Self::Eof,
        }
    }

    fn from_terminal(terminal: &Token) -> Self {
        match terminal {
            Token::String(_) => Self::String,
            Token::Num(_) => Self::Num,
            Token::Bool(_) => Self::Bool,
            Token::LCurly(_) => Self::LCurly,
            Token::RCurly(_) => Self::RCurly,
            Token::LSquare(_) => Self::LSquare,
            Token::RSquare(_) => Self::RSquare,
            Token::Colon(_) => Self::Colon,
            Token::Comma(_) => Self::Comma,
        }
    }
}

impl Node {
    fn from_terminal(terminal: Token) -> Self {
        match terminal {
            Token::String(t) => Self::String(t),
            Token::Num(t) => Self::Num(t),
            Token::Bool(t) => Self::Bool(t),
            Token::LCurly(t) => Self::LCurly(t),
            Token::RCurly(t) => Self::RCurly(t),
            Token::LSquare(t) => Self::LSquare(t),
            Token::RSquare(t) => Self::RSquare(t),
            Token::Colon(t) => Self::Colon(t),
            Token::Comma(t) => Self::Comma(t),
        }
    }
}

impl Quasiterminal {
    fn try_into_terminal(self) -> Result<Token, ()> {
        match self {
            Self::Terminal(terminal) => Ok(terminal),
            Self::Eof => Err(()),
        }
    }
}

const ACTION_TABLE: [[Action; 10]; 26] = [
    [
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Shift(State::S6),
        Action::Err,
        Action::Shift(State::S4),
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
    ],
    [
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R0),
    ],
    [
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R1),
    ],
    [
        Action::Shift(State::S17),
        Action::Shift(State::S18),
        Action::Shift(State::S19),
        Action::Shift(State::S6),
        Action::Err,
        Action::Shift(State::S4),
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
    ],
    [
        Action::Shift(State::S17),
        Action::Shift(State::S18),
        Action::Shift(State::S19),
        Action::Shift(State::S6),
        Action::Err,
        Action::Shift(State::S4),
        Action::Reduce(RuleKind::R14),
        Action::Err,
        Action::Err,
        Action::Err,
    ],
    [
        Action::Shift(State::S17),
        Action::Shift(State::S18),
        Action::Shift(State::S19),
        Action::Shift(State::S6),
        Action::Err,
        Action::Shift(State::S4),
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
    ],
    [
        Action::Shift(State::S13),
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R3),
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
    ],
    [
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Shift(State::S8),
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
    ],
    [
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R2),
        Action::Err,
        Action::Reduce(RuleKind::R2),
        Action::Err,
        Action::Reduce(RuleKind::R2),
        Action::Reduce(RuleKind::R2),
    ],
    [
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R4),
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Shift(State::S11),
        Action::Err,
    ],
    [
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R5),
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R5),
        Action::Err,
    ],
    [
        Action::Shift(State::S13),
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
    ],
    [
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R6),
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R6),
        Action::Err,
    ],
    [
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Shift(State::S3),
        Action::Err,
        Action::Err,
    ],
    [
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R7),
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R7),
        Action::Err,
    ],
    [
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R8),
        Action::Err,
        Action::Reduce(RuleKind::R8),
        Action::Err,
        Action::Reduce(RuleKind::R8),
        Action::Err,
    ],
    [
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R9),
        Action::Err,
        Action::Reduce(RuleKind::R9),
        Action::Err,
        Action::Reduce(RuleKind::R9),
        Action::Err,
    ],
    [
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R10),
        Action::Err,
        Action::Reduce(RuleKind::R10),
        Action::Err,
        Action::Reduce(RuleKind::R10),
        Action::Err,
    ],
    [
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R11),
        Action::Err,
        Action::Reduce(RuleKind::R11),
        Action::Err,
        Action::Reduce(RuleKind::R11),
        Action::Err,
    ],
    [
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R12),
        Action::Err,
        Action::Reduce(RuleKind::R12),
        Action::Err,
        Action::Reduce(RuleKind::R12),
        Action::Err,
    ],
    [
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Shift(State::S21),
        Action::Err,
        Action::Err,
        Action::Err,
    ],
    [
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R13),
        Action::Err,
        Action::Reduce(RuleKind::R13),
        Action::Err,
        Action::Reduce(RuleKind::R13),
        Action::Reduce(RuleKind::R13),
    ],
    [
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R15),
        Action::Err,
        Action::Shift(State::S5),
        Action::Err,
    ],
    [
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R16),
        Action::Err,
        Action::Reduce(RuleKind::R16),
        Action::Err,
    ],
    [
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R17),
        Action::Err,
        Action::Reduce(RuleKind::R17),
        Action::Err,
    ],
    [
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Err,
        Action::Accept,
    ],
];

fn get_action(top_state: State, next_quasiterminal_kind: QuasiterminalKind) -> Action {
    ACTION_TABLE[top_state as usize][next_quasiterminal_kind as usize]
}

const GOTO_TABLE: [[Option<State>; 9]; 26] = [
    [
        Some(State::S25),
        Some(State::S1),
        None,
        None,
        None,
        None,
        Some(State::S2),
        None,
        None,
    ],
    [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [
        None,
        Some(State::S15),
        None,
        None,
        None,
        Some(State::S14),
        Some(State::S16),
        None,
        None,
    ],
    [
        None,
        Some(State::S15),
        None,
        None,
        None,
        Some(State::S23),
        Some(State::S16),
        Some(State::S20),
        Some(State::S22),
    ],
    [
        None,
        Some(State::S15),
        None,
        None,
        None,
        Some(State::S24),
        Some(State::S16),
        None,
        None,
    ],
    [
        None,
        None,
        Some(State::S7),
        Some(State::S9),
        Some(State::S10),
        None,
        None,
        None,
        None,
    ],
    [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [
        None,
        None,
        None,
        None,
        Some(State::S12),
        None,
        None,
        None,
        None,
    ],
    [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
];

fn get_goto(top_state: State, new_node_kind: NonterminalKind) -> Option<State> {
    GOTO_TABLE[top_state as usize][new_node_kind as usize]
}

impl TryFrom<Node> for Json {
    type Error = Node;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Json(n) => Ok(n),
            _ => Err(node),
        }
    }
}

impl TryFrom<Node> for Obj {
    type Error = Node;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Obj(n) => Ok(n),
            _ => Err(node),
        }
    }
}

impl TryFrom<Node> for OptEntries {
    type Error = Node;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::OptEntries(n) => Ok(n),
            _ => Err(node),
        }
    }
}

impl TryFrom<Node> for Entries {
    type Error = Node;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Entries(n) => Ok(n),
            _ => Err(node),
        }
    }
}

impl TryFrom<Node> for Entry {
    type Error = Node;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Entry(n) => Ok(n),
            _ => Err(node),
        }
    }
}

impl TryFrom<Node> for Expr {
    type Error = Node;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Expr(n) => Ok(n),
            _ => Err(node),
        }
    }
}

impl TryFrom<Node> for Arr {
    type Error = Node;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Arr(n) => Ok(n),
            _ => Err(node),
        }
    }
}

impl TryFrom<Node> for OptElements {
    type Error = Node;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::OptElements(n) => Ok(n),
            _ => Err(node),
        }
    }
}

impl TryFrom<Node> for Elements {
    type Error = Node;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Elements(n) => Ok(n),
            _ => Err(node),
        }
    }
}

impl Node {
    fn try_into_string_0(self) -> Result<String, Self> {
        match self {
            Self::String(t) => Ok(t),
            _ => Err(self),
        }
    }
    
    fn try_into_num_1(self) -> Result<String, Self> {
        match self {
            Self::Num(t) => Ok(t),
            _ => Err(self),
        }
    }
    
    fn try_into_bool_2(self) -> Result<String, Self> {
        match self {
            Self::Bool(t) => Ok(t),
            _ => Err(self),
        }
    }
    
    fn try_into_l_curly_3(self) -> Result<String, Self> {
        match self {
            Self::LCurly(t) => Ok(t),
            _ => Err(self),
        }
    }
    
    fn try_into_r_curly_4(self) -> Result<String, Self> {
        match self {
            Self::RCurly(t) => Ok(t),
            _ => Err(self),
        }
    }
    
    fn try_into_l_square_5(self) -> Result<String, Self> {
        match self {
            Self::LSquare(t) => Ok(t),
            _ => Err(self),
        }
    }
    
    fn try_into_r_square_6(self) -> Result<String, Self> {
        match self {
            Self::RSquare(t) => Ok(t),
            _ => Err(self),
        }
    }
    
    fn try_into_colon_7(self) -> Result<String, Self> {
        match self {
            Self::Colon(t) => Ok(t),
            _ => Err(self),
        }
    }
    
    fn try_into_comma_8(self) -> Result<String, Self> {
        match self {
            Self::Comma(t) => Ok(t),
            _ => Err(self),
        }
    }
}