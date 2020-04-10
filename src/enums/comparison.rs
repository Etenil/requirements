use std::fmt;

/// Semver comparison operators
///
/// Abstraction of semver comparison operators.
///
/// # Example
///
/// ```
/// // Get a rule from Pest with a >=
/// let operator = Comparison::from_rule(rule);
/// println!(operator); // -> '>='
/// ```
#[derive(Debug, PartialEq)]
pub enum Comparison {
    LessThan,
    LessThanOrEqual,
    NotEqual,
    Equal,
    GreaterThanOrEqual,
    GreaterThan,
    CompatibleRelease,
    ArbitraryEqual,
}

impl fmt::Display for Comparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Comparison::*;
        let sign = match self {
            LessThan => "<",
            LessThanOrEqual => "<=",
            NotEqual => "!=",
            Equal => "==",
            GreaterThanOrEqual => ">=",
            GreaterThan => ">",
            CompatibleRelease => "~=",
            ArbitraryEqual => "===",
        };
        write!(f, "{}", sign)
    }
}
