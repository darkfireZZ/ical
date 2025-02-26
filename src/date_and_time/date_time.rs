use {
    crate::{Date, Time},
    std::{
        error::Error,
        fmt::{self, Display, Formatter},
        str::FromStr,
    },
};

/// Represents a date and time as specified in
/// [RFC 5545 section 3.3.5](https://tools.ietf.org/html/rfc5545#section-3.3.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTime {
    /// Date component.
    pub date: Date,
    /// Time component.
    pub time: Time,
}

impl FromStr for DateTime {
    type Err = ParseDateTimeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once('T')
            .map_or(Err(ParseDateTimeError {}), |(date, time)| {
                Ok(DateTime {
                    date: date.parse().map_err(|_| ParseDateTimeError {})?,
                    time: time.parse().map_err(|_| ParseDateTimeError {})?,
                })
            })
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}T{}", self.date, self.time)
    }
}

/// Error type for parsing a [`DateTime`].
#[derive(Debug, Clone)]
pub struct ParseDateTimeError {}

impl Display for ParseDateTimeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Invalid date-time")
    }
}

impl Error for ParseDateTimeError {}

#[cfg(test)]
mod tests {
    use {
        super::DateTime,
        crate::{Date, Time},
    };

    #[test]
    fn parse() {
        assert_eq!(
            "19980119T070000Z".parse::<DateTime>().unwrap(),
            DateTime {
                date: Date::new(1998, 1, 19),
                time: Time::new_utc(7, 0, 0),
            }
        );
        // Invalid for this implementation, but technically valid
        assert!("19980118T230000".parse::<DateTime>().is_err());
        assert!("19980119T230000-0800".parse::<DateTime>().is_err());
    }

    #[test]
    fn display() {
        assert_eq!(
            format!(
                "{}",
                DateTime {
                    date: Date::new(1998, 1, 19),
                    time: Time::new_utc(7, 0, 0),
                }
            ),
            "19980119T070000Z"
        );
    }
}
