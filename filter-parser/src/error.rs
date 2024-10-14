use std::fmt::Display;

use nom::error::{self, ParseError};
use nom::Parser;

use crate::{IResult, Span};

pub trait NomErrorExt<E> {
    fn is_failure(&self) -> bool;
    fn map_err<O: FnOnce(E) -> E>(self, op: O) -> nom::Err<E>;
    fn map_fail<O: FnOnce(E) -> E>(self, op: O) -> nom::Err<E>;
}

impl<E> NomErrorExt<E> for nom::Err<E> {
    fn is_failure(&self) -> bool {
        matches!(self, Self::Failure(_))
    }

    fn map_err<O: FnOnce(E) -> E>(self, op: O) -> nom::Err<E> {
        match self {
            e @ Self::Failure(_) => e,
            e => e.map(op),
        }
    }

    fn map_fail<O: FnOnce(E) -> E>(self, op: O) -> nom::Err<E> {
        match self {
            e @ Self::Error(_) => e,
            e => e.map(op),
        }
    }
}

/// cut a parser and map the error
pub fn cut_with_err<'a, O>(
    mut parser: impl FnMut(Span<'a>) -> IResult<'a, O>,
    mut with: impl FnMut(Error<'a>) -> Error<'a>,
) -> impl FnMut(Span<'a>) -> IResult<O> {
    move |input| match parser.parse(input) {
        Err(nom::Err::Error(e)) => Err(nom::Err::Failure(with(e))),
        rest => rest,
    }
}

#[derive(Debug)]
pub struct Error<'a> {
    context: Span<'a>,
    kind: ErrorKind<'a>,
}

#[derive(Debug)]
pub enum ExpectedValueKind {
    ReservedKeyword,
    Other,
}

#[derive(Debug)]
pub enum ErrorKind<'a> {
    ReservedGeo(&'a str),
    GeoRadius,
    GeoBoundingBox,
    MisusedGeoRadius,
    MisusedGeoBoundingBox,
    InvalidPrimary,
    InvalidEscapedNumber,
    ExpectedEof,
    ExpectedValue(ExpectedValueKind),
    MalformedValue,
    InOpeningBracket,
    InClosingBracket,
    NonFiniteFloat,
    InExpectedValue(ExpectedValueKind),
    ReservedKeyword(String),
    MissingClosingDelimiter(char),
    Char(char),
    InternalError(error::ErrorKind),
    DepthLimitReached,
    External(String),
}

impl<'a> Error<'a> {
    pub fn kind(&self) -> &ErrorKind<'a> {
        &self.kind
    }

    pub fn context(&self) -> &Span<'a> {
        &self.context
    }

    pub fn new_from_kind(context: Span<'a>, kind: ErrorKind<'a>) -> Self {
        Self { context, kind }
    }

    pub fn new_from_external(context: Span<'a>, error: impl std::error::Error) -> Self {
        Self::new_from_kind(context, ErrorKind::External(error.to_string()))
    }

    pub fn char(self) -> char {
        match self.kind {
            ErrorKind::Char(c) => c,
            error => panic!("Internal filter parser error: {:?}", error),
        }
    }
}

impl<'a> ParseError<Span<'a>> for Error<'a> {
    fn from_error_kind(input: Span<'a>, kind: error::ErrorKind) -> Self {
        let kind = match kind {
            error::ErrorKind::Eof => ErrorKind::ExpectedEof,
            kind => ErrorKind::InternalError(kind),
        };
        Self {
            context: input,
            kind,
        }
    }

    fn append(_input: Span<'a>, _kind: error::ErrorKind, other: Self) -> Self {
        other
    }

    fn from_char(input: Span<'a>, c: char) -> Self {
        Self {
            context: input,
            kind: ErrorKind::Char(c),
        }
    }
}
