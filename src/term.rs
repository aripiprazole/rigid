/// The location of a term in the source code. This is used for
/// error reporting.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Location {}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Num {
    pub value: isize,
    pub location: Location,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Nil {
    pub location: Option<Location>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Cons {
    pub head: Box<Term>,
    pub tail: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Identifier {
    pub name: String,
    pub location: Location,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Atom {
    pub name: String,
    pub location: Location,
}

/// The basic expression structure in the rigid programming language
/// is a term. A term is either a number, a nil, a cons, an atom, or
/// an identifier.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Term {
    Nil(Nil),
    Num(Num),
    Cons(Cons),
    Atom(Atom),
    Identifier(Identifier),
}

impl Term {
    /// Returns the location of the term in the source code.
    pub fn location(&self) -> Option<Location> {
        match self {
            Term::Nil(n) => n.location.clone(),
            Term::Num(n) => Some(n.location.clone()),
            Term::Cons(c) => Some(c.location.clone()),
            Term::Atom(a) => Some(a.location.clone()),
            Term::Identifier(i) => Some(i.location.clone()),
        }
    }
}