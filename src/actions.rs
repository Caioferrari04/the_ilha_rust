use crate::time::TimeOfDay;

pub fn hunt(time_of_day: TimeOfDay) -> u16 {
    match time_of_day {
        TimeOfDay::Madrugada => 4,
        TimeOfDay::Manha => 2,
        TimeOfDay::Tarde => 1,
        TimeOfDay::Noite => 5
    }
}

pub fn gather_fruits(time_of_day: TimeOfDay) -> u16 {
    match time_of_day {
        TimeOfDay::Madrugada => 2,
        TimeOfDay::Manha => 1,
        TimeOfDay::Tarde => 1,
        TimeOfDay::Noite => 2
    }
}

pub fn scavenge(time_of_day: TimeOfDay) -> u16 {
    match time_of_day {
        TimeOfDay::Madrugada => 2,
        TimeOfDay::Manha => 1,
        TimeOfDay::Tarde => 1,
        TimeOfDay::Noite => 2
    }
}

pub fn treatment(time_of_day: TimeOfDay) -> u16 {
    match time_of_day {
        TimeOfDay::Madrugada => 2,
        TimeOfDay::Manha => 1,
        TimeOfDay::Tarde => 1,
        TimeOfDay::Noite => 2
    }
}

pub fn sleep(time_of_day: TimeOfDay) -> u16 {
    match time_of_day {
        TimeOfDay::Madrugada => 6,
        TimeOfDay::Manha => 4,
        TimeOfDay::Tarde => 2,
        TimeOfDay::Noite => 8
    }
}
