// This code was generated by Kiki.
// Kiki is an open-source minimalist parser generator for Rust.
// You can read more at https://crates.io/crates/kiki
//
// This code was generated from a grammar with the following hash:
// @sha256 f6176f7ea7b906a84d040468932bed981a6503efa72c10983e3c44be453ec264

// Since this code is automatically generated,
// some parts may be unidiomatic.
// The linter often complains about these parts.
// However, these warnings are not useful.
// Therefore, we disable certain lints for this file.
#![allow(non_snake_case)]
#![allow(dead_code)]

pub enum Token {
    String(String),
    Number(isize),
}

pub enum Foo {
    Empty,
    Number,
    Pair {
        val: Box<Pair>,
    },
}

pub struct Epsilon;

pub enum Pair {
    StringPair(
        Box<StringPair>,
    ),
    NumberPair(
        Box<NumberPair>,
    ),
}

pub struct StringPair;

pub struct NumberPair;

/// If the parser encounters an unexpected token `t`, it will return `Err(Some(t))`.
/// If the parser encounters an unexpected end of input, it will return `Err(None)`.
pub fn parse<S>(src: S) -> Result<Foo, Option<Token>>
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
                return Ok(Foo::try_from(nodes.pop().unwrap()).ok().unwrap());
            }

            Action::Err => {
                return Err(quasiterminals.next().unwrap().try_into_terminal().ok());
            }
        }
    }
}

enum Quasiterminal {
    Terminal(Token),
    Eof,
}

#[derive(Clone, Copy, Debug)]
enum QuasiterminalKind {
    String = 0,
    Number = 1,
    Eof = 2,
}

#[derive(Clone, Copy, Debug)]
enum NonterminalKind {
    Foo = 0,
    Epsilon = 1,
    Pair = 2,
    StringPair = 3,
    NumberPair = 4,
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
}

enum Node {
    Foo(Foo),
    Epsilon(Epsilon),
    Pair(Pair),
    StringPair(StringPair),
    NumberPair(NumberPair),
    String(String),
    Number(isize),
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
}

fn pop_and_reduce(states: &mut Vec<State>, nodes: &mut Vec<Node>, rule_kind: RuleKind) -> (Node, NonterminalKind) {
    match rule_kind {
        RuleKind::R0 => {
            nodes.pop().unwrap();
            
            states.truncate(states.len() - 1);
            
            (
                Node::Foo(Foo::Empty),
                NonterminalKind::Foo,
            )
        }
        RuleKind::R1 => {
            nodes.pop().unwrap();
            
            states.truncate(states.len() - 1);
            
            (
                Node::Foo(Foo::Number),
                NonterminalKind::Foo,
            )
        }
        RuleKind::R2 => {
            let val_0 = Box::new(Pair::try_from(nodes.pop().unwrap()).ok().unwrap());
            
            states.truncate(states.len() - 1);
            
            (
                Node::Foo(Foo::Pair {
                    val: val_0,
                }),
                NonterminalKind::Foo,
            )
        }
        RuleKind::R3 => {
            (
                Node::Epsilon(Epsilon),
                NonterminalKind::Epsilon,
            )
        }
        RuleKind::R4 => {
            let t0 = Box::new(StringPair::try_from(nodes.pop().unwrap()).ok().unwrap());
            
            states.truncate(states.len() - 1);
            
            (
                Node::Pair(Pair::StringPair(
                    t0,
                )),
                NonterminalKind::Pair,
            )
        }
        RuleKind::R5 => {
            let t0 = Box::new(NumberPair::try_from(nodes.pop().unwrap()).ok().unwrap());
            
            states.truncate(states.len() - 1);
            
            (
                Node::Pair(Pair::NumberPair(
                    t0,
                )),
                NonterminalKind::Pair,
            )
        }
        RuleKind::R6 => {
            nodes.pop().unwrap();
            nodes.pop().unwrap();
            
            states.truncate(states.len() - 2);
            
            (
                Node::StringPair(StringPair),
                NonterminalKind::StringPair,
            )
        }
        RuleKind::R7 => {
            nodes.pop().unwrap();
            nodes.pop().unwrap();
            
            states.truncate(states.len() - 2);
            
            (
                Node::NumberPair(NumberPair),
                NonterminalKind::NumberPair,
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
            Token::Number(_) => Self::Number,
        }
    }
}

impl Node {
    fn from_terminal(terminal: Token) -> Self {
        match terminal {
            Token::String(t) => Self::String(t),
            Token::Number(t) => Self::Number(t),
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

const ACTION_TABLE: [[Action; 3]; 10] = [
    [
        Action::Shift(State::S6),
        Action::Shift(State::S2),
        Action::Reduce(RuleKind::R3),
    ],
    [
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R0),
    ],
    [
        Action::Err,
        Action::Shift(State::S8),
        Action::Reduce(RuleKind::R1),
    ],
    [
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R2),
    ],
    [
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R4),
    ],
    [
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R5),
    ],
    [
        Action::Shift(State::S7),
        Action::Err,
        Action::Err,
    ],
    [
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R6),
    ],
    [
        Action::Err,
        Action::Err,
        Action::Reduce(RuleKind::R7),
    ],
    [
        Action::Err,
        Action::Err,
        Action::Accept,
    ],
];

fn get_action(top_state: State, next_quasiterminal_kind: QuasiterminalKind) -> Action {
    ACTION_TABLE[top_state as usize][next_quasiterminal_kind as usize]
}

const GOTO_TABLE: [[Option<State>; 5]; 10] = [
    [
        Some(State::S9),
        Some(State::S1),
        Some(State::S3),
        Some(State::S4),
        Some(State::S5),
    ],
    [
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
    ],
    [
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
    ],
    [
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
    ],
    [
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
    ],
    [
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

impl TryFrom<Node> for Foo {
    type Error = Node;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Foo(n) => Ok(n),
            _ => Err(node),
        }
    }
}

impl TryFrom<Node> for Epsilon {
    type Error = Node;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Epsilon(n) => Ok(n),
            _ => Err(node),
        }
    }
}

impl TryFrom<Node> for Pair {
    type Error = Node;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Pair(n) => Ok(n),
            _ => Err(node),
        }
    }
}

impl TryFrom<Node> for StringPair {
    type Error = Node;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::StringPair(n) => Ok(n),
            _ => Err(node),
        }
    }
}

impl TryFrom<Node> for NumberPair {
    type Error = Node;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::NumberPair(n) => Ok(n),
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
    
    fn try_into_number_1(self) -> Result<isize, Self> {
        match self {
            Self::Number(t) => Ok(t),
            _ => Err(self),
        }
    }
}