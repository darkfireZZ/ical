use {
    ical_vcard::{Contentline, Value},
    std::io::{self, Write},
    uuid::Uuid,
};

mod date_and_time;
pub use date_and_time::{Date, DateTime, ParseDateError, ParseDateTimeError, ParseTimeError, Time};

mod recurrence_rule;
pub use recurrence_rule::{RecurrenceFrequency, RecurrenceRule};

/// Default product identifier.
pub const DEFAULT_PRODUCT_IDENTIFIER: &str = concat!(
    "nicolabruhin.com : ",
    env!("CARGO_PKG_NAME"),
    " ",
    env!("CARGO_PKG_VERSION")
);

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
    pub fn set_product_identifier<S: Into<String>>(&mut self, product_identifier: S) {
        self.product_identifier =
            Some(Value::new(product_identifier.into()).unwrap_or_else(|err| {
                panic!("Invalid product identifier: {}", err);
            }));
    }

    /// Get the product identifier of the calendar.
    ///
    /// By default, the product identifier is set to [`DEFAULT_PRODUCT_IDENTIFIER`].
    ///
    /// See [RFC 5545 section 3.7.3 - Product Identifier](https://tools.ietf.org/html/rfc5545#section-3.7.3)
    /// for more information.
    pub fn product_identifier(&self) -> &str {
        self.product_identifier
            .as_ref()
            .map_or(DEFAULT_PRODUCT_IDENTIFIER, |s| s.as_str())
    }

    // TODO: Improve doc (errors, BufWriter)
    /// Write the calendar to the given writer.
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

#[derive(Debug, Clone)]
pub enum Component {
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

#[derive(Debug, Clone)]
pub struct Event {
    /// Corresponds to the `UID` property.
    ///
    /// See [RFC 5545 section 3.8.4.7 - Unique Identifier](https://www.rfc-editor.org/rfc/rfc5545#section-3.8.4.7)
    uid: Value<String>,
    /// Corresponds to the `DTSTAMP` property.
    ///
    /// See [RFC 5545 section 3.8.7.2 - Date-Time
    /// Stamp](https://www.rfc-editor.org/rfc/rfc5545#section-3.8.7.2)
    date_time: DateTime,
    /// Corresponds to the `RRULE` property.
    ///
    /// See [RFC 5545 section 3.8.5.3 - Recurrence
    /// Rule](https://www.rfc-editor.org/rfc/rfc5545#section-3.8.5.3)
    recurrence_rule: Option<RecurrenceRule>,
}

impl Event {
    /// Create a new [`Event`].
    ///
    /// The `UID` property is automatically set to a random UUID (v4).
    pub fn new(date_time: DateTime) -> Self {
        Self {
            uid: Value::new(Uuid::new_v4().to_string()).expect("UUIDs are always valid values"),
            date_time,
            recurrence_rule: None,
        }
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
        writer.write(&Contentline::new("DTSTAMP", self.date_time.to_string()))?;
        writer.write(&Contentline::new("UID", self.uid.as_str()))?;
        if let Some(recurrence_rule) = &self.recurrence_rule {
            writer.write(&Contentline::new("RRULE", recurrence_rule.to_string()))?;
        }
        writer.write(&Contentline::new("END", "VEVENT"))?;
        Ok(())
    }
}
