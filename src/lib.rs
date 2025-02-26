#![doc = include_str!("../README.md")]
#![warn(
    clippy::cargo,
    clippy::pedantic,
    clippy::absolute_paths,
    clippy::allow_attributes_without_reason,
    clippy::dbg_macro,
    clippy::exit,
    clippy::todo,
    clippy::unimplemented,
    clippy::unwrap_used,
    missing_debug_implementations,
    missing_docs
)]
// The following lints are enable by default in clippy::pedantic, but are disabled here because
// they are too aggressive.
#![allow(clippy::module_name_repetitions, reason = "Occasionally useful")]
#![allow(clippy::too_many_lines, reason = "This is not bad in my opinion")]

use {
    ical_vcard::{Contentline, Value},
    std::io::{self, Write},
    uuid::Uuid,
};

mod date_and_time;
pub use date_and_time::{Date, DateTime, ParseDateError, ParseDateTimeError, ParseTimeError, Time};

mod recurrence_rule;
pub use recurrence_rule::{RecurrenceFrequency, RecurrenceRule};

mod start_date_time;
pub use start_date_time::StartDateTime;

/// Default product identifier.
pub const DEFAULT_PRODUCT_IDENTIFIER: &str = concat!(
    "nicolabruhin.com : ",
    env!("CARGO_PKG_NAME"),
    " ",
    env!("CARGO_PKG_VERSION")
);

/// Represents an iCalendar object.
///
/// > The body of the iCalendar object consists of a sequence of calendar properies and
/// > one or more calendar components.
///
/// For more information, see
/// - [RFC 5545 section 3 - iCalendar Object
///   Specification](https://tools.ietf.org/html/rfc5545#section-3)
/// - and [RFC 5545 section 3.4 - iCalendar Object](https://tools.ietf.org/html/rfc5545#section-3.4)
#[derive(Debug, Clone)]
pub struct Calendar {
    /// Corresponds to the `PRODID` property.
    ///
    /// See [RFC 5545 section 3.7.3 - Product
    /// Identifier](https://tools.ietf.org/html/rfc5545#section-3.7.3)
    product_identifier: Option<Value<String>>,
    components: Vec<Component>,
}

impl Calendar {
    #[allow(
        clippy::new_without_default,
        reason = "The default value is not deterministic"
    )]
    /// Create a new [`Calendar`] object.
    #[must_use]
    pub fn new() -> Self {
        Calendar {
            product_identifier: None,
            components: Vec::new(),
        }
    }

    /// Set the product identifier of the calendar.
    ///
    /// By default, the product identifier is set to [`DEFAULT_PRODUCT_IDENTIFIER`].
    ///
    /// See [RFC 5545 section 3.7.3 - Product Identifier](https://tools.ietf.org/html/rfc5545#section-3.7.3)
    /// for more information.
    ///
    /// # Panics
    ///
    /// Panics if the product identifier is not a valid [`Value`].
    pub fn set_product_identifier<S: Into<String>>(&mut self, product_identifier: S) -> &mut Self {
        self.product_identifier =
            Some(Value::new(product_identifier.into()).unwrap_or_else(|err| {
                panic!("Invalid product identifier: {err}");
            }));
        self
    }

    /// Get the product identifier of the calendar.
    ///
    /// By default, the product identifier is set to [`DEFAULT_PRODUCT_IDENTIFIER`].
    ///
    /// See [RFC 5545 section 3.7.3 - Product Identifier](https://tools.ietf.org/html/rfc5545#section-3.7.3)
    /// for more information.
    #[must_use]
    pub fn product_identifier(&self) -> &str {
        self.product_identifier
            .as_ref()
            .map_or(DEFAULT_PRODUCT_IDENTIFIER, |s| s.as_str())
    }

    /// Add a [`Component`] to the calendar.
    pub fn add_component<C: Into<Component>>(&mut self, component: C) -> &mut Self {
        self.components.push(component.into());
        self
    }

    /// Get the [`Component`]s of the calendar.
    #[must_use]
    pub fn components(&self) -> &[Component] {
        &self.components
    }

    /// Write the calendar to the given writer.
    ///
    /// It is advisable to pass a buffered writer such as [`std::io::BufWriter`] to this function.
    /// This will likely improve performance significantly by reducing the number of write
    /// operations. See [`std::io::BufWriter`] for more information.
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the writer fails.
    pub fn write<W: Write>(&self, writer: W) -> io::Result<()> {
        let mut writer = ical_vcard::Writer::new(writer);
        writer.write(&Contentline::new("BEGIN", "VCALENDAR"))?;
        writer.write(&Contentline::new("PRODID", self.product_identifier()))?;
        writer.write(&Contentline::new("VERSION", "2.0"))?;
        for component in &self.components {
            component.write(&mut writer)?;
        }
        writer.write(&Contentline::new("END", "VCALENDAR"))?;
        Ok(())
    }
}

/// Represents a component of a calendar.
///
/// Citing from [RFC 5545 section 3.6 - Calendar
/// Components](https://tools.ietf.org/html/rfc5545#section-3.6):
/// > The body of the iCalendar object consists of a sequence of calendar
/// > properties and one or more calendar components.  The calendar
/// > properties are attributes that apply to the calendar object as a
/// > whole.  The calendar components are collections of properties that
/// > express a particular calendar semantic.  For example, the calendar
/// > component can specify an event, a to-do, a journal entry, time zone
/// > information, free/busy time information, or an alarm.
///
#[derive(Debug, Clone)]
pub enum Component {
    /// An event component.
    Event(Event),
}

impl Component {
    /// Write the component to the given writer.
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the writer fails.
    fn write<W: Write>(&self, writer: &mut ical_vcard::Writer<W>) -> io::Result<()> {
        match self {
            Component::Event(event) => event.write(writer),
        }
    }
}

impl From<Event> for Component {
    fn from(event: Event) -> Self {
        Component::Event(event)
    }
}

/// Represents an event component of a calendar.
///
/// See [RFC 5545 section 3.6.1 - Event
/// Component](https://tools.ietf.org/html/rfc5545#section-3.6.1)
#[derive(Debug, Clone)]
pub struct Event {
    /// Corresponds to the `UID` property.
    ///
    /// See [RFC 5545 section 3.8.4.7 - Unique
    /// Identifier](https://tools.ietf.org/html/rfc5545#section-3.8.4.7)
    uid: Value<String>,
    /// Corresponds to the `DTSTAMP` property.
    ///
    /// See [RFC 5545 section 3.8.7.2 - Date-Time
    /// Stamp](https://tools.ietf.org/html/rfc5545#section-3.8.7.2)
    date_time: DateTime,
    /// Corresponds to the `DTSTART` property.
    ///
    /// See [RFC 5545 section 3.8.2.4 - Date-Time
    /// Start](https://tools.ietf.org/html/rfc5545#section-3.8.2.4)
    start_date_time: StartDateTime,
    /// Corresponds to the `DESCRIPTION` property.
    ///
    /// See [RFC 5545 section 3.8.1.5 -
    /// Description](https://tools.ietf.org/html/rfc5545#section-3.8.1.5)
    description: Option<Value<String>>,
    /// Corresponds to the `LOCATION` property.
    ///
    /// See [RFC 5545 section 3.8.1.7 -
    /// Location](https://tools.ietf.org/html/rfc5545#section-3.8.1.7)
    location: Option<Value<String>>,
    /// Corresponds to the `SUMMARY` property.
    ///
    /// See [RFC 5545 section 3.8.1.12 -
    /// Summary](https://tools.ietf.org/html/rfc5545#section-3.8.1.12)
    summary: Option<Value<String>>,
    /// Corresponds to the `RRULE` property.
    ///
    /// See [RFC 5545 section 3.8.5.3 - Recurrence
    /// Rule](https://tools.ietf.org/html/rfc5545#section-3.8.5.3)
    recurrence_rule: Option<RecurrenceRule>,
}

impl Event {
    /// Create a new [`Event`].
    ///
    /// The `UID` property is automatically set to a random UUID (v4).
    #[must_use]
    #[allow(clippy::missing_panics_doc, reason = "This will never panic")]
    pub fn new(start_date_time: StartDateTime, date_time: DateTime) -> Self {
        Self {
            uid: Value::new(Uuid::new_v4().to_string()).expect("UUIDs are always valid values"),
            date_time,
            start_date_time,
            description: None,
            location: None,
            summary: None,
            recurrence_rule: None,
        }
    }

    /// Set the description of the event.
    ///
    /// # Panics
    ///
    /// Panics if `description` is not a valid [`Value`].
    pub fn set_description<S: Into<String>>(&mut self, description: S) {
        self.description = Some(Value::new(description.into()).unwrap_or_else(|err| {
            panic!("Invalid description: {err}");
        }));
    }

    /// Set the location of the event.
    ///
    /// # Panics
    ///
    /// Panics if `location` is not a valid [`Value`].
    pub fn set_location<S: Into<String>>(&mut self, location: S) {
        self.location = Some(Value::new(location.into()).unwrap_or_else(|err| {
            panic!("Invalid location: {err}");
        }));
    }

    /// Set the summary for the event.
    ///
    /// # Panics
    ///
    /// Panics if `summary` is not a valid [`Value`].
    pub fn set_summary<S: Into<String>>(&mut self, summary: S) {
        self.summary = Some(Value::new(summary.into()).unwrap_or_else(|err| {
            panic!("Invalid summary: {err}");
        }));
    }

    /// Set a recurrence rule for the event.
    pub fn set_recurrence_rule(&mut self, recurrence_rule: RecurrenceRule) {
        self.recurrence_rule = Some(recurrence_rule);
    }

    /// Write the event to the given writer.
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the writer fails.
    fn write<W: Write>(&self, writer: &mut ical_vcard::Writer<W>) -> io::Result<()> {
        writer.write(&Contentline::new("BEGIN", "VEVENT"))?;
        writer.write(&Contentline::new("UID", self.uid.as_str()))?;
        writer.write(&Contentline::new("DTSTAMP", self.date_time.to_string()))?;
        self.start_date_time.write(writer)?;
        if let Some(description) = &self.description {
            writer.write(&Contentline::new("DESCRIPTION", description.as_str()))?;
        }
        if let Some(location) = &self.location {
            writer.write(&Contentline::new("LOCATION", location.as_str()))?;
        }
        if let Some(summary) = &self.summary {
            writer.write(&Contentline::new("SUMMARY", summary.as_str()))?;
        }
        if let Some(recurrence_rule) = &self.recurrence_rule {
            writer.write(&Contentline::new("RRULE", recurrence_rule.to_string()))?;
        }
        writer.write(&Contentline::new("END", "VEVENT"))?;
        Ok(())
    }
}
