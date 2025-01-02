impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let input = self.context.fragment();
        // When printing our error message we want to escape all `\n` to be sure we keep our format with the
        // first line being the diagnostic and the second line being the incriminated filter.
        let escaped_input = input.escape_debug();

        match &self.kind {
        ErrorKind::ExpectedValue(_) if input.trim().is_empty() => {
            writeln!(f, "Was expecting a value but instead got nothing.")?
        }
        ErrorKind::ExpectedValue(ExpectedValueKind::ReservedKeyword) => {
            writeln!(f, "Was expecting a value but instead got `{escaped_input}`, which is a reserved keyword. To use `{escaped_input}` as a field name or a value, surround it by quotes.")?
        }
        ErrorKind::ExpectedValue(ExpectedValueKind::Other) => {
            writeln!(f, "Was expecting a value but instead got `{}`.", escaped_input)?
        }
        ErrorKind::MalformedValue => {
            writeln!(f, "Malformed value: `{}`.", escaped_input)?
        }
        ErrorKind::MissingClosingDelimiter(c) => {
            writeln!(f, "Expression `{}` is missing the following closing delimiter: `{}`.", escaped_input, c)?
        }
        ErrorKind::InvalidPrimary => {
            let text = if input.trim().is_empty() { "but instead got nothing.".to_string() } else { format!("at `{}`.", escaped_input) };
            writeln!(f, "Was expecting an operation `=`, `!=`, `>=`, `>`, `<=`, `<`, `IN`, `NOT IN`, `TO`, `EXISTS`, `NOT EXISTS`, `IS NULL`, `IS NOT NULL`, `IS EMPTY`, `IS NOT EMPTY`, `CONTAINS`, `NOT CONTAINS`, `STARTS WITH`, `NOT STARTS WITH`, `_geoRadius`, or `_geoBoundingBox` {}", text)?
        }
        ErrorKind::InvalidEscapedNumber => {
            writeln!(f, "Found an invalid escaped sequence number: `{}`.", escaped_input)?
        }
        ErrorKind::ExpectedEof => {
            writeln!(f, "Found unexpected characters at the end of the filter: `{}`. You probably forgot an `OR` or an `AND` rule.", escaped_input)?
        }
        ErrorKind::GeoRadius => {
            writeln!(f, "The `_geoRadius` filter expects three arguments: `_geoRadius(latitude, longitude, radius)`.")?
        }
        ErrorKind::GeoBoundingBox => {
            writeln!(f, "The `_geoBoundingBox` filter expects two pairs of arguments: `_geoBoundingBox([latitude, longitude], [latitude, longitude])`.")?
        }
        ErrorKind::ReservedGeo(name) => {
            writeln!(f, "`{}` is a reserved keyword and thus can't be used as a filter expression. Use the `_geoRadius(latitude, longitude, distance)` or `_geoBoundingBox([latitude, longitude], [latitude, longitude])` built-in rules to filter on `_geo` coordinates.", name.escape_debug())?
        }
        ErrorKind::MisusedGeoRadius => {
            writeln!(f, "The `_geoRadius` filter is an operation and can't be used as a value.")?
        }
        ErrorKind::MisusedGeoBoundingBox => {
            writeln!(f, "The `_geoBoundingBox` filter is an operation and can't be used as a value.")?
        }
        ErrorKind::ReservedKeyword(word) => {
            writeln!(f, "`{word}` is a reserved keyword and thus cannot be used as a field name unless it is put inside quotes. Use \"{word}\" or \'{word}\' instead.")?
        }
        ErrorKind::InOpeningBracket => {
            writeln!(f, "Expected `[` after `IN` keyword.")?
        }
        ErrorKind::InClosingBracket => {
            writeln!(f, "Expected matching `]` after the list of field names given to `IN[`")?
        }
        ErrorKind::NonFiniteFloat => {
            writeln!(f, "Non finite floats are not supported")?
        }
        ErrorKind::InExpectedValue(ExpectedValueKind::ReservedKeyword) => {
            writeln!(f, "Expected only comma-separated field names inside `IN[..]` but instead found `{escaped_input}`, which is a keyword. To use `{escaped_input}` as a field name or a value, surround it by quotes.")?
        }
        ErrorKind::InExpectedValue(ExpectedValueKind::Other) => {
            writeln!(f, "Expected only comma-separated field names inside `IN[..]` but instead found `{escaped_input}`.")?
        }
        ErrorKind::Char(c) => {
            panic!("Tried to display a char error with `{}`", c)
        }
        ErrorKind::DepthLimitReached => writeln!(
            f,
            "The filter exceeded the maximum depth limit. Try rewriting the filter so that it contains fewer nested conditions."
        )?,
        ErrorKind::InternalError(kind) => writeln!(
            f,
            "Encountered an internal `{:?}` error while parsing your filter. Please fill an issue", kind
        )?,
        ErrorKind::External(ref error) => writeln!(f, "{}", error)?,
    }
        let base_column = self.context.get_utf8_column();
        let size = self.context.fragment().chars().count();

        write!(
            f,
            "{}:{} {}",
            base_column,
            base_column + size,
            self.context.extra
        )
    }
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let input = self.context.fragment();
        // When printing our error message we want to escape all `\n` to be sure we keep our format with the
        // first line being the diagnostic and the second line being the incriminated filter.
        let escaped_input = input.escape_debug();

        match &self.kind {
        ErrorKind::ExpectedValue(_) if input.trim().is_empty() => {
            writeln!(f, "Was expecting a value but instead got nothing.")?
        }
        ErrorKind::ExpectedValue(ExpectedValueKind::ReservedKeyword) => {
            writeln!(f, "Was expecting a value but instead got `{escaped_input}`, which is a reserved keyword. To use `{escaped_input}` as a field name or a value, surround it by quotes.")?
        }
        ErrorKind::ExpectedValue(ExpectedValueKind::Other) => {
            writeln!(f, "Was expecting a value but instead got `{}`.", escaped_input)?
        }
        ErrorKind::MalformedValue => {
            writeln!(f, "Malformed value: `{}`.", escaped_input)?
        }
        ErrorKind::MissingClosingDelimiter(c) => {
            writeln!(f, "Expression `{}` is missing the following closing delimiter: `{}`.", escaped_input, c)?
        }
        ErrorKind::InvalidPrimary => {
            let text = if input.trim().is_empty() { "but instead got nothing.".to_string() } else { format!("at `{}`.", escaped_input) };
            writeln!(f, "Was expecting an operation `=`, `!=`, `>=`, `>`, `<=`, `<`, `IN`, `NOT IN`, `TO`, `EXISTS`, `NOT EXISTS`, `IS NULL`, `IS NOT NULL`, `IS EMPTY`, `IS NOT EMPTY`, `CONTAINS`, `NOT CONTAINS`, `STARTS WITH`, `NOT STARTS WITH`, `_geoRadius`, or `_geoBoundingBox` {}", text)?
        }
        ErrorKind::InvalidEscapedNumber => {
            writeln!(f, "Found an invalid escaped sequence number: `{}`.", escaped_input)?
        }
        ErrorKind::ExpectedEof => {
            writeln!(f, "Found unexpected characters at the end of the filter: `{}`. You probably forgot an `OR` or an `AND` rule.", escaped_input)?
        }
        ErrorKind::GeoRadius => {
            writeln!(f, "The `_geoRadius` filter expects three arguments: `_geoRadius(latitude, longitude, radius)`.")?
        }
        ErrorKind::GeoBoundingBox => {
            writeln!(f, "The `_geoBoundingBox` filter expects two pairs of arguments: `_geoBoundingBox([latitude, longitude], [latitude, longitude])`.")?
        }
        ErrorKind::ReservedGeo(name) => {
            writeln!(f, "`{}` is a reserved keyword and thus can't be used as a filter expression. Use the `_geoRadius(latitude, longitude, distance)` or `_geoBoundingBox([latitude, longitude], [latitude, longitude])` built-in rules to filter on `_geo` coordinates.", name.escape_debug())?
        }
        ErrorKind::MisusedGeoRadius => {
            writeln!(f, "The `_geoRadius` filter is an operation and can't be used as a value.")?
        }
        ErrorKind::MisusedGeoBoundingBox => {
            writeln!(f, "The `_geoBoundingBox` filter is an operation and can't be used as a value.")?
        }
        ErrorKind::ReservedKeyword(word) => {
            writeln!(f, "`{word}` is a reserved keyword and thus cannot be used as a field name unless it is put inside quotes. Use \"{word}\" or \'{word}\' instead.")?
        }
        ErrorKind::InOpeningBracket => {
            writeln!(f, "Expected `[` after `IN` keyword.")?
        }
        ErrorKind::InClosingBracket => {
            writeln!(f, "Expected matching `]` after the list of field names given to `IN[`")?
        }
        ErrorKind::NonFiniteFloat => {
            writeln!(f, "Non finite floats are not supported")?
        }
        ErrorKind::InExpectedValue(ExpectedValueKind::ReservedKeyword) => {
            writeln!(f, "Expected only comma-separated field names inside `IN[..]` but instead found `{escaped_input}`, which is a keyword. To use `{escaped_input}` as a field name or a value, surround it by quotes.")?
        }
        ErrorKind::InExpectedValue(ExpectedValueKind::Other) => {
            writeln!(f, "Expected only comma-separated field names inside `IN[..]` but instead found `{escaped_input}`.")?
        }
        ErrorKind::Char(c) => {
            panic!("Tried to display a char error with `{}`", c)
        }
        ErrorKind::DepthLimitReached => writeln!(
            f,
            "The filter exceeded the maximum depth limit. Try rewriting the filter so that it contains fewer nested conditions."
        )?,
        ErrorKind::InternalError(kind) => writeln!(
            f,
            "Encountered an internal `{:?}` error while parsing your filter. Please fill an issue", kind
        )?,
        ErrorKind::External(ref error) => writeln!(f, "{}", error)?,
    }
        let base_column = self.context.get_utf8_column();
        let size = self.context.fragment().chars().count();

        write!(
            f,
            "{}:{} {}",
            base_column,
            base_column + size,
            self.context.extra
        )
    }
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


mpl<E> NomErrorExt<E> for nom::Err<E> {
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