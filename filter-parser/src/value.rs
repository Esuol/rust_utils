use nom::branch::alt;
use nom::bytes::complete::{take_till, take_while, take_while1};
use nom::character::complete::{char, multispace0};
use nom::combinator::cut;
use nom::sequence::{delimited, terminated};
use nom::{InputIter, InputLength, InputTake, Slice};

use crate::error::{ExpectedValueKind, NomErrorExt};
use crate::{
    parse_geo, parse_geo_bounding_box, parse_geo_distance, parse_geo_point, parse_geo_radius,
    Error, ErrorKind, IResult, Span, Token,
};

/// This function goes through all characters in the [Span] if it finds any escaped character (`\`).
/// It generates a new string with all `\` removed from the [Span].
fn unescape(buf: Span, char_to_escape: char) -> String {
    let to_escape = format!("\\{}", char_to_escape);
    buf.replace(&to_escape, &char_to_escape.to_string())
}
