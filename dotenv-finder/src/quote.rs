use dotenv_core::is_escaped;

pub(crate) enum Quote {
    Single,
    Double,
}

impl Quote {
    pub(crate) fn as_char(&self) -> char {
        match self {
            Quote::Single => '\'',
            Quote::Double => '\"',
        }
    }

    fn is_quoted(&self, val: &str) -> bool {
        let Some(rest) = val.strip_prefix(self.as_char()) else {
            return false;
        };

        // A closed value may have an inline comment after its closing quote.
        !rest
            .char_indices()
            .any(|(index, ch)| ch == self.as_char() && !is_escaped(&rest[..index]))
    }
}

/// Returns the `Quote` for a value whose leading quote has not been closed yet.
pub(crate) fn get_quote(val: &str) -> Option<Quote> {
    [Quote::Single, Quote::Double]
        .into_iter()
        .find(|q| q.is_quoted(val))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_quoted() {
        assert!(Quote::Single.is_quoted("\'some_quoted_str"))
    }

    #[test]
    fn test_double_quoted() {
        assert!(Quote::Double.is_quoted("\"some_quoted_str"))
    }

    #[test]
    fn test_non_single_quoted() {
        assert!(!Quote::Single.is_quoted("some_non_quoted_str"))
    }

    #[test]
    fn test_non_double_quoted() {
        assert!(!Quote::Double.is_quoted("some_non_quoted_str"))
    }

    #[test]
    fn test_single_quoted_for_double_quoted_str() {
        assert!(!Quote::Single.is_quoted("\"some_double_quoted_str"))
    }

    #[test]
    fn test_double_quoted_for_single_quoted_str() {
        assert!(!Quote::Double.is_quoted("\'some_single_quoted_str"))
    }

    #[test]
    fn closed_quotes_with_inline_comments_do_not_start_multiline_values() {
        for value in [
            r#""two words" # comment"#,
            r#""has \" # inside" # comment"#,
            r#""ends with \\" # comment"#,
        ] {
            assert!(!Quote::Double.is_quoted(value), "{value}");
        }
        assert!(!Quote::Single.is_quoted("'two words' # comment"));
    }

    #[test]
    fn hashes_and_escaped_quotes_do_not_close_multiline_values() {
        for value in [r#""two # words"#, r#""two \" # words"#] {
            assert!(Quote::Double.is_quoted(value), "{value}");
        }
        assert!(Quote::Single.is_quoted("'two # words"));
    }
}
