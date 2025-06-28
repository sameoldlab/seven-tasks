use iced::{widget, Element, Length::Fill, Theme};
use std::fmt::{Display, Error, Formatter};
use time::{macros::format_description, Date, OffsetDateTime};

fn main() -> iced::Result {
    iced::application("Flit", update, view)
        .centered()
        .theme(theme)
        .run()
}

pub struct Fb {
    triptype: TripType,
    departure_date: String,
    arrival_date: String,
    error: Option<BookingError>,
    message: String,
}

impl Default for Fb {
    fn default() -> Self {
        let now = OffsetDateTime::now_utc().date();
        Fb {
            triptype: TripType::Oneway,
            departure_date: format!("{now}"),
            arrival_date: format!("{now}"),
            error: None,
            message: String::new(),
        }
    }
}
impl Fb {
    fn reset(&mut self) {
        let now = OffsetDateTime::now_utc().date();
        self.departure_date = now.to_string();
        self.arrival_date = now.to_string();
        self.error = None;
        self.message = String::new();
        self.triptype = TripType::Oneway;
    }
    fn validate_booking(&self) -> Result<(), BookingError> {
        let format = format_description!("[year]-[month]-[day]");
        let now = time::OffsetDateTime::now_utc().date();
        if self.triptype == TripType::Oneway {
            let departure = Date::parse(&self.departure_date, &format)
                .map_err(|err| BookingError::InvalidDateFormat(Field::Departure, err))?;
            if (departure - now).is_negative() {
                return Err(BookingError::PastDeparture);
            }
            Ok(())
        } else {
            let departure = Date::parse(&self.departure_date, &format)
                .map_err(|err| BookingError::InvalidDateFormat(Field::Departure, err))?;
            let arrival = Date::parse(&self.arrival_date, &format)
                .map_err(|err| BookingError::InvalidDateFormat(Field::Arrival, err))?;

            if (departure - now).is_negative() {
                return Err(BookingError::PastDeparture);
            };
            if (arrival - departure).is_negative() {
                return Err(BookingError::ArrivalBeforeDeparture);
            }
            Ok(())
        }
    }
}
#[derive(Debug)]
enum Field {
    Departure,
    Arrival,
}
impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(match self {
            Self::Departure => "Departure",
            Self::Arrival => "Arrival",
        })
    }
}

#[derive(Debug)]
enum BookingError {
    PastDeparture,
    InvalidDateFormat(Field, time::error::Parse),
    ArrivalBeforeDeparture,
}
impl Display for BookingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::PastDeparture => write!(f, "Departure must be in the future"),
            Self::InvalidDateFormat(field, parse) => write!(f, "Error parsing {field}: {parse}"),
            Self::ArrivalBeforeDeparture => write!(f, "Cannot arrive before departure"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TripType {
    Oneway,
    Roundtrip,
}

impl Display for TripType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(match self {
            Self::Oneway => "One way",
            Self::Roundtrip => "Round trip",
        })
    }
}

impl Default for TripType {
    fn default() -> Self {
        Self::Oneway
    }
}

#[derive(Debug, Clone)]
enum Message {
    TriptypeChanged(TripType),
    DepartureUpdated(String),
    ArrivalUpdated(String),
    Submit,
}
fn theme(_state: &Fb) -> Theme {
    Theme::CatppuccinMocha
}
fn update(state: &mut Fb, message: Message) {
    match message {
        Message::TriptypeChanged(trip_type) => {
            state.triptype = trip_type;
        }
        Message::DepartureUpdated(date) => state.departure_date = date,
        Message::ArrivalUpdated(date) => state.arrival_date = date,
        Message::Submit => {
            state.reset();
            state.message = format!("");
        }
    }
    state.error = if let Err(error) = state.validate_booking() {
        Some(error)
    } else {
        None
    };
}
fn view(state: &Fb) -> Element<Message> {
    let flights = [TripType::Oneway, TripType::Roundtrip];
    let arrival =
        widget::text_input("", &state.arrival_date).on_input_maybe(match state.triptype {
            TripType::Roundtrip => Some(|input| Message::ArrivalUpdated(input)),
            TripType::Oneway => None,
        });
    let error = widget::text(
        state
            .error
            .as_ref()
            .map_or(String::new(), |err| err.to_string()),
    )
    .color(theme(state).palette().danger);
    widget::container(
        widget::column![
            widget::pick_list(flights, Some(state.triptype), Message::TriptypeChanged),
            widget::row![
                widget::text("depart").width(60),
                widget::text_input("", &state.departure_date).on_input(Message::DepartureUpdated)
            ],
            widget::row![widget::text("arrive").width(60), arrival],
            widget::button("Book").on_press_maybe(match state.error {
                None => Some(Message::Submit),
                Some(_) => None,
            }),
            error,
            widget::text(&state.message).color(theme(state).palette().success)
        ]
        .spacing(8),
    )
    .center_x(Fill)
    .into()
}
