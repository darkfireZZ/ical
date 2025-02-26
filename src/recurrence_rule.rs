use {
    crate::DateTime,
    std::{
        fmt::{Display, Error, Formatter},
        str::FromStr,
    },
};

/// Represents a recurrence rule as specified in
/// [RFC 5545 section 3.3.10](https://tools.ietf.org/html/rfc5545#section-3.3.10).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecurrenceRule {
    freq: RecurrenceFrequency,
    until: Option<DateTime>,
}

impl RecurrenceRule {
    /// Create a new recurrence rule with the specified frequency.
    pub fn new(freq: RecurrenceFrequency) -> Self {
        RecurrenceRule { freq, until: None }
    }

    /// Set the end date (inclusive) of the recurrence rule.
    pub fn until(&self, until: DateTime) -> Self {
        RecurrenceRule {
            freq: self.freq,
            until: Some(until),
        }
    }
}

impl Display for RecurrenceRule {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "FREQ={}", self.freq)?;
        if let Some(until) = self.until {
            write!(f, ";UNTIL={}", until)?;
        }
        Ok(())
    }
}

/// The frequency of a recurrence rule.
///
/// This is used to specify how often a recurrence rule should repeat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecurrenceFrequency {
    /// Repeat in intervals measured in years.
    Yearly,
    /// Repeat in intervals measured in months.
    Monthly,
    /// Repeat in intervals measured in weeks.
    Weekly,
    /// Repeat in intervals measured in days.
    Daily,
    /// Repeat in intervals measured in hours.
    Hourly,
    /// Repeat in intervals measured in minutes.
    Minutely,
    /// Repeat in intervals measured in seconds.
    Secondly,
}

impl Display for RecurrenceFrequency {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            RecurrenceFrequency::Yearly => write!(f, "YEARLY"),
            RecurrenceFrequency::Monthly => write!(f, "MONTHLY"),
            RecurrenceFrequency::Weekly => write!(f, "WEEKLY"),
            RecurrenceFrequency::Daily => write!(f, "DAILY"),
            RecurrenceFrequency::Hourly => write!(f, "HOURLY"),
            RecurrenceFrequency::Minutely => write!(f, "MINUTELY"),
            RecurrenceFrequency::Secondly => write!(f, "SECONDLY"),
        }
    }
}

impl FromStr for RecurrenceFrequency {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "YEARLY" => Ok(RecurrenceFrequency::Yearly),
            "MONTHLY" => Ok(RecurrenceFrequency::Monthly),
            "WEEKLY" => Ok(RecurrenceFrequency::Weekly),
            "DAILY" => Ok(RecurrenceFrequency::Daily),
            "HOURLY" => Ok(RecurrenceFrequency::Hourly),
            "MINUTELY" => Ok(RecurrenceFrequency::Minutely),
            "SECONDLY" => Ok(RecurrenceFrequency::Secondly),
            _ => Err(()),
        }
    }
}
