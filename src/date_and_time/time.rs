use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

/// Represents a time as specified in
/// [RFC 5545 section 3.3.12](https://tools.ietf.org/html/rfc5545#section-3.3.12).
///
/// [RFC 5545](https://tools.ietf.org/html/rfc5545#section-3.3.12) specifies that a time may be in
/// either local time or UTC time. At the moment, only UTC time is supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Time {
    hour: u8,
    minute: u8,
    second: u8,
}

impl Time {
    /// Create a new `Time`.
    ///
    /// # Panics
    ///
    /// Panics in any of the following cases:
    /// - `hour` is greater than 23.
    /// - `minute` is greater than 59.
    /// - `second` is greater than 59.
    ///
    /// Note that RFC 5545 technically allows for leap seconds, that is, a `second` value of 60.
    /// However, this is currently not supported.
    pub fn new_utc(hour: u8, minute: u8, second: u8) -> Self {
        if hour > 23 {
            panic!("Hour must be between 0 and 23");
        } else if minute > 59 {
            panic!("Minute must be between 0 and 59");
        } else if second > 59 {
            panic!("Second must be between 0 and 59");
        }
        Self {
            hour,
            minute,
            second,
        }
    }

    /// Get the hour.
    pub fn hour(&self) -> u8 {
        self.hour
    }

    /// Get the minute.
    pub fn minute(&self) -> u8 {
        self.minute
    }

    /// Get the second.
    pub fn second(&self) -> u8 {
        self.second
    }
}

impl FromStr for Time {
    type Err = ParseTimeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 7 {
            return Err(ParseTimeError {});
        }
        let bytes = s.as_bytes();
        if !bytes[0..6].iter().all(|&b| b.is_ascii_digit()) {
            return Err(ParseTimeError {});
        }
        if bytes[6] != b'Z' {
            return Err(ParseTimeError {});
        }
        let hour = s[0..2].parse().map_err(|_| ParseTimeError {})?;
        if hour > 23 {
            return Err(ParseTimeError {});
        }
        let minute = s[2..4].parse().map_err(|_| ParseTimeError {})?;
        if minute > 59 {
            return Err(ParseTimeError {});
        }
        let second = s[4..6].parse().map_err(|_| ParseTimeError {})?;
        if second > 59 {
            return Err(ParseTimeError {});
        }
        Ok(Self {
            hour,
            minute,
            second,
        })
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:02}{:02}{:02}Z", self.hour, self.minute, self.second)
    }
}

/// Error type for parsing a [`Time`].
#[derive(Debug, Clone)]
pub struct ParseTimeError {}

impl Display for ParseTimeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Invalid time")
    }
}

impl Error for ParseTimeError {}

#[cfg(test)]
mod tests {
    use super::Time;

    #[test]
    fn parse() {
        assert_eq!(
            "235959Z".parse::<Time>().unwrap(),
            Time::new_utc(23, 59, 59)
        );
        assert_eq!("000000Z".parse::<Time>().unwrap(), Time::new_utc(0, 0, 0));
        assert_eq!("120000Z".parse::<Time>().unwrap(), Time::new_utc(12, 0, 0));
        assert_eq!(
            "123456Z".parse::<Time>().unwrap(),
            Time::new_utc(12, 34, 56)
        );
        assert!("240000Z".parse::<Time>().is_err());
        assert!("006000Z".parse::<Time>().is_err());
        assert!("000060Z".parse::<Time>().is_err());
        assert!("235960Z".parse::<Time>().is_err());
        assert!("123456".parse::<Time>().is_err());
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", Time::new_utc(23, 59, 59)), "235959Z");
        assert_eq!(format!("{}", Time::new_utc(0, 0, 0)), "000000Z");
        assert_eq!(format!("{}", Time::new_utc(12, 0, 0)), "120000Z");
        assert_eq!(format!("{}", Time::new_utc(12, 34, 56)), "123456Z");
    }

    #[test]
    fn compare() {
        assert_eq!(Time::new_utc(23, 4, 5), Time::new_utc(23, 4, 5));
        assert!(Time::new_utc(13, 0, 1) > Time::new_utc(12, 0, 3));
        assert!(Time::new_utc(17, 20, 35) < Time::new_utc(17, 35, 20));
    }

    #[test]
    #[should_panic]
    fn invalid_hour() {
        Time::new_utc(24, 0, 0);
    }

    #[test]
    #[should_panic]
    fn invalid_minute() {
        Time::new_utc(0, 60, 0);
    }

    #[test]
    #[should_panic]
    fn invalid_second() {
        Time::new_utc(0, 0, 60);
    }
}
