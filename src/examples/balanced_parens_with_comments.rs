// This code was generated by Kiki.
// Kiki is an open-source minimalist parser generator for Rust.
// You can read more at https://crates.io/crates/kiki
//
// This code was generated from a grammar with the following hash:
// @sha256 25ca8e28fd01ed600ed0fefeada5c1bf5a206d793903395854c5ab3836eff5f5

// Since this code is automatically generated,
// some parts may be unidiomatic.
// The linter often complains about these parts.
// However, these warnings are not useful.
// Therefore, we disable certain lints for this file.
#![allow(non_snake_case)]
#![allow(dead_code)]

#[derive(Debug)]
pub enum Token {
    LParen(()),
    RParen(()),
}

#[derive(Clone, Debug)]
pub enum Expr {
    Empty,
    Wrap(
        Box<Expr>,
    ),
}

/// If the parser encounters an unexpected token `t`, it will return `Err(Some(t))`.
/// If the parser encounters an unexpected end of input, it will return `Err(None)`.
pub fn parse<S>(src: S) -> Result<Expr, Option<Token>>
where S: IntoIterator<Item = Token> {
    let mut quasiterminals = src.into_iter()
        .map(Quasiterminal::Terminal)
        .chain(std::iter::once(Quasiterminal::Eof))
        .peekable();
    let mut states = vec![State::S1];
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
                return Ok(Expr::try_from(nodes.pop().unwrap()).unwrap());
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
    LParen = 0,
    RParen = 1,
    Eof = 2,
}

#[derive(Clone, Copy, Debug)]
enum NonterminalKind {
    Expr = 0,
}

#[derive(Clone, Copy, Debug)]
enum State {
    S0 = 0,
    S1 = 1,
    S2 = 2,
    S3 = 3,
    S4 = 4,
}

#[derive(Debug)]
enum Node {
    Expr(Expr),
    LParen(()),
    RParen(()),
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
}

fn pop_and_reduce(states: &mut Vec<State>, nodes: &mut Vec<Node>, rule_kind: RuleKind) -> (Node, NonterminalKind) {
    match rule_kind {
        RuleKind::R0 => {
            (
                Node::Expr(Expr::Empty),
                NonterminalKind::Expr,
            )
        }
        RuleKind::R1 => {
            nodes.pop().unwrap();
            let t1 = Box::new(Expr::try_from(nodes.pop().unwrap()).unwrap());
            nodes.pop().unwrap();
            
            states.truncate(states.len() - 3);
            
            (
                Node::Expr(Expr::Wrap(
                    t1,
                )),
                NonterminalKind::Expr,
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
            Token::LParen(_) => Self::LParen,
            Token::RParen(_) => Self::RParen,
        }
    }
}

impl Node {
    fn from_terminal(terminal: Token) -> Self {
        match terminal {
            Token::LParen(t) => Self::LParen(t),
            Token::RParen(t) => Self::RParen(t),
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

const ACTION_TABLE: [[Action; 3]; 5] = [
    [
        Action::Shift(State::S0),
        Action::Reduce(RuleKind::R0),
        Action::Err,
    ],
    [
        Action::Shift(State::S0),
        Action::Err,
        Action::Reduce(RuleKind::R0),
    ],
    [
        Action::Err,
        Action::Shift(State::S3),
        Action::Err,
    ],
    [
        Action::Err,
        Action::Reduce(RuleKind::R1),
        Action::Reduce(RuleKind::R1),
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

const GOTO_TABLE: [[Option<State>; 1]; 5] = [
    [
        Some(State::S2),
    ],
    [
        Some(State::S4),
    ],
    [
        None,
    ],
    [
        None,
    ],
    [
        None,
    ],
];

fn get_goto(top_state: State, new_node_kind: NonterminalKind) -> Option<State> {
    GOTO_TABLE[top_state as usize][new_node_kind as usize]
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

impl Node {
    fn try_into_l_paren_0(self) -> Result<(), Self> {
        match self {
            Self::LParen(t) => Ok(t),
            _ => Err(self),
        }
    }
    
    fn try_into_r_paren_1(self) -> Result<(), Self> {
        match self {
            Self::RParen(t) => Ok(t),
            _ => Err(self),
        }
    }
}
