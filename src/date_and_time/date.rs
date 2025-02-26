use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

/// Represents a date as specified in
/// [RFC 5545 section 3.3.4](https://tools.ietf.org/html/rfc5545#section-3.3.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

/// Returns true if the given year is a leap year.
fn is_leap_year(year: u16) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Returns the number of days in the given month of the given year.
///
/// # Panics
///
/// If month is not in the range 1-12.
fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => panic!("Invalid month: {month}"),
    }
}

impl Date {
    /// Create a new `Date`.
    ///
    /// # Panics
    ///
    /// Panics in any of the following cases:
    /// - `year` is greater than 9999.
    /// - `month` is not in the range 1-12.
    /// - `day` is not in the valid range for the given month and year.
    #[must_use]
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        if year > 9999 {
            panic!("Year must be less than 10000");
        } else if !(1..=12).contains(&month) {
            panic!("Month must be between 1 and 12");
        } else if day < 1 || day > days_in_month(year, month) {
            panic!("Day ({day}) out of range for month ({month})");
        }
        Self { year, month, day }
    }

    /// Get the year of the date.
    #[must_use]
    pub fn year(&self) -> u16 {
        self.year
    }

    /// Set the year of the date.
    ///
    /// # Panics
    ///
    /// Panics if the given year is greater than 9999.
    pub fn set_year(&mut self, year: u16) {
        assert!((year <= 9999), "Year must be less than 10000");
        self.year = year;
    }

    /// Get the month of the date.
    #[must_use]
    pub fn month(&self) -> u8 {
        self.month
    }

    /// Set the month of the date.
    ///
    /// # Panics
    ///
    /// Panics if the given month is not in the range 1-12.
    pub fn set_month(&mut self, month: u8) {
        assert!((1..=12).contains(&month), "Month must be between 1 and 12");
        self.month = month;
    }

    /// Get the day of the date.
    #[must_use]
    pub fn day(&self) -> u8 {
        self.day
    }

    /// Set the day of the date.
    ///
    /// # Panics
    ///
    /// Panics if the given day is not in the valid range for the current month and year.
    pub fn set_day(&mut self, day: u8) {
        assert!(
            !(day < 1 || day > days_in_month(self.year, self.month)),
            "Day ({day}) out of range for month ({})",
            self.month
        );
        self.day = day;
    }
}

impl FromStr for Date {
    type Err = ParseDateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 8 {
            return Err(ParseDateError {});
        }
        let bytes = s.as_bytes();
        if !bytes.iter().all(|&b| b.is_ascii_digit()) {
            return Err(ParseDateError {});
        }
        let year = s[0..4].parse().map_err(|_| ParseDateError {})?;
        if year > 9999 {
            return Err(ParseDateError {});
        }
        let month = s[4..6].parse().map_err(|_| ParseDateError {})?;
        if !(1..=12).contains(&month) {
            return Err(ParseDateError {});
        }
        let day = s[6..8].parse().map_err(|_| ParseDateError {})?;
        if day < 1 || day > days_in_month(year, month) {
            return Err(ParseDateError {});
        }
        Ok(Self { year, month, day })
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:04}{:02}{:02}", self.year, self.month, self.day)
    }
}

/// Error type for parsing a [`Date`].
#[derive(Debug, Clone)]
pub struct ParseDateError {}

impl Display for ParseDateError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Invalid date")
    }
}

impl Error for ParseDateError {}

#[cfg(test)]
mod tests {
    use super::Date;

    #[test]
    fn parse() {
        assert_eq!("99990909".parse::<Date>().unwrap(), Date::new(9999, 9, 9));
        assert_eq!("20210101".parse::<Date>().unwrap(), Date::new(2021, 1, 1));
        assert_eq!("20200229".parse::<Date>().unwrap(), Date::new(2020, 2, 29));
        assert!("20200230".parse::<Date>().is_err());
        assert!("20211301".parse::<Date>().is_err());
        assert!("20211232".parse::<Date>().is_err());
        assert!("2021-01-01".parse::<Date>().is_err());
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", Date::new(2021, 1, 1)), "20210101");
        assert_eq!(format!("{}", Date::new(0, 7, 7)), "00000707");
    }

    #[test]
    fn compare() {
        assert_eq!(Date::new(2021, 1, 1), Date::new(2021, 1, 1));
        assert!(Date::new(2022, 1, 1) > Date::new(2021, 1, 3));
        assert!(Date::new(2021, 2, 3) < Date::new(2021, 3, 2));
    }

    #[test]
    #[should_panic]
    fn invalid_year() {
        let _ = Date::new(10000, 2, 30);
    }

    #[test]
    #[should_panic]
    fn invalid_month_1() {
        let _ = Date::new(2021, 13, 1);
    }

    #[test]
    #[should_panic]
    fn invalid_month_2() {
        let _ = Date::new(2021, 0, 1);
    }

    #[test]
    #[should_panic]
    fn invalid_day_1() {
        let _ = Date::new(2021, 1, 32);
    }

    #[test]
    #[should_panic]
    fn invalid_day_2() {
        let _ = Date::new(2021, 2, 29);
    }

    #[test]
    #[should_panic]
    fn invalid_day_3() {
        let _ = Date::new(2021, 4, 0);
    }
}
