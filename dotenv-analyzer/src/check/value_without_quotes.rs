use dotenv_core::{LineEntry, is_escaped};

use super::Check;
use crate::{LintKind, Warning};

pub(crate) struct ValueWithoutQuotesChecker<'a> {
    template: &'a str,
}

impl ValueWithoutQuotesChecker<'_> {
    fn message(&self) -> &str {
        self.template
    }
}

impl Default for ValueWithoutQuotesChecker<'_> {
    fn default() -> Self {
        Self {
            template: "This value needs to be surrounded in quotes",
        }
    }
}

fn value_without_comment(value: &str) -> &str {
    let value = value.trim();
    let mut quote = None;

    for (index, ch) in value.char_indices() {
        // Only a leading quote delimits the value; internal quotes may be literal.
        if index == 0 && matches!(ch, '\'' | '"') {
            quote = Some(ch);
        } else if Some(ch) == quote && !is_escaped(&value[..index]) {
            quote = None;
        } else if ch == '#' && quote.is_none() {
            return value[..index].trim_end();
        }
    }

    value
}

impl Check for ValueWithoutQuotesChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let val = value_without_comment(line.get_value()?);

        if val.contains(char::is_whitespace)
            && !(val.starts_with('\'') && val.ends_with('\''))
            && !(val.starts_with('\"') && val.ends_with('\"'))
        {
            Some(Warning::new(line.number, self.name(), self.message()))
        } else {
            None
        }
    }

    fn name(&self) -> LintKind {
        LintKind::ValueWithoutQuotes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::check_test;

    const WARNING: &str = "This value needs to be surrounded in quotes";

    #[test]
    fn value_without_quotes() {
        check_test(
            &mut ValueWithoutQuotesChecker::default(),
            [
                ("FOO=BAR", None),
                ("FOO=BAR BAZ", Some(WARNING)),
                ("FOO=\"BAR BAZ\"", None),
                ("FOO=\'BAR BAR\'", None),
            ],
        );
    }

    #[test]
    fn inline_comments() {
        check_test(
            &mut ValueWithoutQuotesChecker::default(),
            [
                ("FOO=BAR # comment", None),
                ("FOO=BAR# comment", None),
                ("FOO= # empty value", None),
                ("FOO=# empty value", None),
                ("FOO=BAR\t# comment with spaces", None),
                ("FOO=BAR BAZ # comment", Some(WARNING)),
                ("FOO=BAR\tBAZ # comment", Some(WARNING)),
                ("FOO=BAR BAZ# comment", Some(WARNING)),
            ],
        );
    }

    #[test]
    fn quoted_values_with_inline_comments() {
        check_test(
            &mut ValueWithoutQuotesChecker::default(),
            [
                (r#"FOO="BAR BAZ" # comment"#, None),
                ("FOO='BAR BAZ' # comment", None),
                (r#"FOO="BAR # BAZ" # comment"#, None),
                ("FOO='BAR # BAZ' # comment", None),
                (r#"FOO="BAR # BAZ""#, None),
                ("FOO='BAR # BAZ'", None),
                (r#"FOO="BAR" BAZ # comment"#, Some(WARNING)),
                ("FOO='BAR' BAZ # comment", Some(WARNING)),
                (r#"FOO="BAR BAZ # unclosed quote"#, Some(WARNING)),
                ("FOO='BAR BAZ # unclosed quote", Some(WARNING)),
            ],
        );
    }

    #[test]
    fn escaped_quotes_with_inline_comments() {
        check_test(
            &mut ValueWithoutQuotesChecker::default(),
            [
                (r#"FOO="BAR \" # BAZ" # comment"#, None),
                (r#"FOO='BAR \' # BAZ' # comment"#, None),
                (r#"FOO="BAR\\" # comment"#, None),
                (r#"FOO='BAR\\' # comment"#, None),
                (r#"FOO="BAR\\\" # BAZ" # comment"#, None),
                (r#"FOO="BAR\\" BAZ # comment"#, Some(WARNING)),
            ],
        );
    }

    #[test]
    fn literal_quotes_in_unquoted_values() {
        check_test(
            &mut ValueWithoutQuotesChecker::default(),
            [
                ("FOO=can't # comment", None),
                (r#"FOO=some"text # comment"#, None),
                ("FOO=can't stop # comment", Some(WARNING)),
            ],
        );
    }

    #[test]
    fn unicode_values_with_inline_comments() {
        check_test(
            &mut ValueWithoutQuotesChecker::default(),
            [
                ("FOO=é# comment", None),
                ("FOO=€ # comment", None),
                ("FOO=🦀 # comment", None),
                ("FOO=日本語 # comment", None),
                ("FOO=olá mundo # comentário", Some(WARNING)),
                ("FOO=\"olá # mundo\" # comentário", None),
            ],
        );
    }
}
