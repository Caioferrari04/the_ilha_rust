use std::fmt;

pub enum TimeOfDay {
    Madrugada,
    Manha,
    Tarde,
    Noite
}

impl fmt::Display for TimeOfDay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TimeOfDay::Madrugada => write!(f, "período da madrugada"),
            TimeOfDay::Manha => write!(f, "período da manha"),
            TimeOfDay::Tarde => write!(f, "período da tarde"),
            TimeOfDay::Noite => write!(f, "período da noite"),
        }
    }
}

pub fn process_time(current_time: u16) -> u16 {
    if current_time > 24 {
        current_time - 24
    } else {
        current_time
    }
}

pub fn time_of_day(&current_time: &u16) -> TimeOfDay {
    match current_time {
        1..=6 => TimeOfDay::Madrugada,
        7..=12 => TimeOfDay::Manha,
        13..=18 => TimeOfDay::Tarde,
        19..=24 => TimeOfDay::Noite,
        _ => TimeOfDay::Madrugada
    }
}

pub fn to_string(&processed_time: &u16) -> String {
    processed_time.to_string() + if processed_time > 1 { " horas" } else { " hora" }
}
