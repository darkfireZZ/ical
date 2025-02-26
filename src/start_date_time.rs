use {
    crate::{Date, DateTime},
    ical_vcard::Contentline,
    std::{
        fmt,
        fmt::Display,
        io::{self, Write},
    },
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StartDateTime {
    value: DateAndMaybeTime,
}

impl StartDateTime {
    pub(crate) fn write<W: Write>(&self, writer: &mut ical_vcard::Writer<W>) -> io::Result<()> {
        writer.write(&Contentline::new("DTSTART", self.value.to_string()))
    }
}

impl From<Date> for StartDateTime {
    fn from(date: Date) -> Self {
        Self {
            value: DateAndMaybeTime::Date(date),
        }
    }
}

impl From<DateTime> for StartDateTime {
    fn from(date_time: DateTime) -> Self {
        Self {
            value: DateAndMaybeTime::DateTime(date_time),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum DateAndMaybeTime {
    Date(Date),
    DateTime(DateTime),
}

impl Display for DateAndMaybeTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DateAndMaybeTime::Date(date) => write!(f, "{date}"),
            DateAndMaybeTime::DateTime(date_time) => write!(f, "{date_time}"),
        }
    }
}
