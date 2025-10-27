// use std::time::Duration;
use time::{OffsetDateTime,Date, macros::format_description};
pub use time;
pub mod crud;

/* #[derive(Default)]
pub struct AppState {
    tab: Tabs,
    num: i32,
    /// temperature in celsius
    temp: Option<f32>,
    elapsed_time: f64,
    total_time: f64,
    flight_booker: FlightBooker,
} */

#[repr(usize)]
#[derive(Default, Clone, Copy, PartialEq)]
pub enum Tabs {
    #[default]
    Counter,
    TempConverter,
    FlightBooker,
    Timer,
    Crud,
    Circles,
    Cells,
}

/***********************
      @TEMPCONVERTER
************************/

pub fn from_fahrenheit(f: f32) -> f32 {
    (f - 32.0) * (5.0 / 9.0)
}
pub fn to_fahrenheit(c: f32) -> f32 {
    c * 9.0 / 5.0 + 32.0
}

/***********************
      @FLIGHTBOOKER
************************/

pub struct FlightBooker {
    pub trip_type: TripType,
    pub departure: Date,
    pub arrival: Date,
    // departure_error: Option<String>,
    // arrival_error: Option<String>,
    // arrival_error: Option<String>,
    // msg: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TripType {
    OneWay,
    Return,
}
impl std::fmt::Display for TripType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TripType::OneWay => write!(f, "One-way Trip"),
            TripType::Return => write!(f, "Round Trip"),
        }
    }
}

#[derive(Debug)]
pub enum AppointmentError {
    Parse(u8),
    PastDate(u8),
}

impl Default for FlightBooker {
    fn default() -> Self {
        // let format = format_description!("[year]-[month]-[day]");
        let now = OffsetDateTime::now_utc().date();
        // let now_string = now.format(format).unwrap();
        Self {
            trip_type: TripType::OneWay,
            departure: now,
            arrival: now,
        }
    }
}
impl FlightBooker {
    // let format = format_description!("[year]-[month]-[day]");
    pub fn validate(&self) -> Vec<AppointmentError> {
        let mut errors: Vec<AppointmentError> = Vec::with_capacity(4);
        let mut min = OffsetDateTime::now_utc().date();
        if (self.departure - min).is_negative() {
            errors.push(AppointmentError::PastDate(0));
        }
        if self.trip_type == TripType::Return {
            min = self.departure;
            if (self.arrival - min).is_negative() {
                errors.push(AppointmentError::PastDate(1));
            }
        }
        errors
    }
    pub fn reset(&mut self) {
        let now = OffsetDateTime::now_utc().date();
        self.trip_type = TripType::OneWay;
        self.departure = now;
        self.arrival = now;
    }
}


pub fn fmt_date(v: String) -> Result<Date, String> {
    let format = format_description!("[year]-[month]-[day]");
    match Date::parse(&v, &format) {
        Ok(d) => Ok(d),
        Err(e) => Err(e.to_string()),
    }
}
