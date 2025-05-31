use winnow::ascii::{alphanumeric1, multispace0, multispace1, space1};
use winnow::combinator::seq;
use winnow::token::take_until;
use winnow::{Parser, Result};

use super::Backend;

//TODO: Add escaping support
pub fn string_literal<'s>(s: &mut &'s str) -> Result<&'s str> {
    let (_, value, _) = ('"', take_until(0.., '"'), '"').parse_next(s)?;
    Ok(value)
}

pub fn parse_backend(input: &mut &str) -> Result<Backend> {
    seq!(
        Backend {
            _: ("backend", space1),
            name: alphanumeric1.map(|s: &str| s.to_owned()),
            _: (multispace1, '{', multispace1),
            _: (".host = "),
            host: string_literal.map(|s: &str| s.to_owned()),
            _: (multispace0, ";", multispace0, "}")
        }
    )
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use winnow::error::{ContextError, ErrMode};

    use super::*;

    #[test]
    fn strings() -> Result<(), ContextError> {
        let sample = "\"blah\"";

        let (remaining, result) = string_literal.parse_peek(sample)?;

        assert!(remaining.is_empty());
        assert_eq!("blah", result);
        Ok(())
    }

    #[test]
    fn parse_basic() -> Result<(), ContextError> {
        let basic_backend = "backend default {
            .host = \"google.com:443\";
        }";

        let (remaining, backend) = parse_backend.parse_peek(basic_backend)?;

        assert!(remaining.is_empty());
        assert_eq!(
            backend,
            Backend {
                name: "default".to_string(),
                host: "google.com:443".to_string()
            }
        );

        Ok(())
    }
}
