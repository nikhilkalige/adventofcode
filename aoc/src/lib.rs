use std::error::Error;
use std::fmt::{Display, Formatter};
use std::result::Result;

pub trait Parser<'a> {
    type Output;

    fn parse(&self, input: &'a str) -> Result<Self::Output, Box<dyn Error>>;

    fn is_default(&self) -> bool {
        false
    }
}

pub trait Solver<I> {
    type Output;

    fn solve(&self, input: I) -> Result<Self::Output, Box<dyn Error>>;

    fn is_implemented(&self) -> bool {
        true
    }
}

pub trait ParserDefault {}

impl<'a, T> Parser<'a> for &T
where
    T: ParserDefault,
{
    type Output = &'a str;

    fn parse(&self, input: &'a str) -> Result<Self::Output, Box<dyn Error>> {
        Ok(input)
    }

    fn is_default(&self) -> bool {
        true
    }
}

pub trait SolverDefault {
    type Input;
}

impl<I, T> Solver<I> for &T
where
    T: SolverDefault<Input = I>,
{
    type Output = ();

    fn solve(&self, _: I) -> Result<Self::Output, Box<dyn Error>> {
        Err(Box::new(NotImplemented))
    }

    fn is_implemented(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct NotImplemented;

impl Display for NotImplemented {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Not Implemented")
    }
}

impl Error for NotImplemented {}

#[derive(Debug)]
pub struct ParserFailed;

impl Display for ParserFailed {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Parser Failed")
    }
}

impl Error for ParserFailed {}
