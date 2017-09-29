use prelude::*;
use poly::PolyError;
use std::fmt;
use lalrpop_util;

#[derive(Debug)]
pub enum Error {
    MissingFunction(String),
    ParseError {
        token: String,
        pos: usize,
        expected: Vec<String>,
        input: String
    },
    IntegerError,
    Poly(PolyError),
    Undefined(String),
    ShapeMismatch(usize, usize),
    Todo(&'static str),
    Bug(&'static str),
    Other(String),
    Overflow
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        
        match *self {
            MissingFunction(ref s) => write!(f, "the function '{}' is not implemented", s),
            ParseError { pos, ref token, ref expected, ref input } => 
                write!(f, "the token «{}» was not one of the expected {}: {}\u{32d}{}", token, expected.iter().join(" ,"), &input[..pos], &input[pos..]),
            IntegerError => write!(f, "not an integer"),
            Poly(PolyError::DivZero) => write!(f, "division by zero"),
            Undefined(ref name) => write!(f, "'{}' is not defined", name),
            ShapeMismatch(a, b) => write!(f, "shapes do not match ({} vs. {})", a, b),
            Todo(what) => write!(f, "{} is not implemented yet", what),
            Bug(what) => write!(f, "BUG: {}", what),
            Other(ref msg) => write!(f, "{}", msg),
            Overflow => write!(f, "out of bits!")
        }       
    }
}
impl From<PolyError> for Error {
    fn from(e: PolyError) -> Error { Error::Poly(e) }
}
impl Error {
    pub fn parse_error(e: lalrpop_util::ParseError<usize, (usize, &str), ()>, input: &str) -> Error {
        use lalrpop_util::ParseError::UnrecognizedToken;
        match e {
            UnrecognizedToken { token: Some((pos, (_, span), _end)), expected } =>
                Error::ParseError { pos, token: span.into(), expected, input: input.into() },
            e => Error::Other(format!("Other({:?})", e))
        }
    }
}
