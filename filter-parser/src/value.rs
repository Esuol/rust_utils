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

fn quoted_by(quote: char, input: Span) -> IResult<Token> {
    // empty fields / values are valid in json
    if input.is_empty() {
        return Ok((input.slice(input.input_len()..), input.into()));
    }

    let mut escaped = false;
    let mut i = input.iter_indices();

    while let Some((idx, c)) = i.next() {
        if c == quote {
            let (rem, output) = input.take_split(idx);
            return Ok((
                rem,
                Token::new(output, escaped.then(|| unescape(output, quote))),
            ));
        } else if c == '\\' {
            if let Some((_, c)) = i.next() {
                escaped |= c == quote;
            } else {
                return Err(nom::Err::Error(Error::new_from_kind(
                    input,
                    ErrorKind::MalformedValue,
                )));
            }
        }
        // if it was preceded by a `\` or if it was anything else we can continue to advance
    }

    Ok((
        input.slice(input.input_len()..),
        Token::new(input, escaped.then(|| unescape(input, quote))),
    ))
}

// word           = (alphanumeric | _ | - | .)+    except for reserved keywords
pub fn word_not_keyword<'a>(input: Span<'a>) -> IResult<Token<'a>> {
    let (input, word): (_, Token<'a>) =
        take_while1(is_value_component)(input).map(|(s, t)| (s, t.into()))?;
    if is_keyword(word.value()) {
        return Err(nom::Err::Error(Error::new_from_kind(
            input,
            ErrorKind::ReservedKeyword(word.value().to_owned()),
        )));
    }
    Ok((input, word))
}
