use std::fmt::Display;

use nom::error::{self, ParseError};
use nom::Parser;

use crate::{IResult, Span};

pub trait NomErrorExt<E> {
    fn is_failure(&self) -> bool;
    fn map_err<O: FnOnce(E) -> E>(self, op: O) -> nom::Err<E>;
    fn map_fail<O: FnOnce(E) -> E>(self, op: O) -> nom::Err<E>;
}
