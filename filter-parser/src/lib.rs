use std::fmt::Error;

use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str, &'a str>;

type IResult<'a, Ret> = nom::IResult<Span<'a>, Ret, Error<'a>>;