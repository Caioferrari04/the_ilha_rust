use std::fmt;
use rand::Rng;
use crate::time::TimeOfDay;

pub struct Protagonist {
    pub hp: Option<Status>,
    pub hunger: Option<Status>,
    pub sleep: Option<Status>
}

#[derive(FromPrimitive, Copy, Clone)]
pub enum Status {
    Horrible = 1, //%15
    Bad = 2, //%25
    Normal = 3, //%40
    Good = 4, //%10
    Excelent = 5 //%5
}

pub struct Event {
    pub description: String,
    pub quality: Option<Status>,
    pub time_spent: u8,
    pub is_dead: bool
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Horrible => write!(f, " horrível!"),
            Status::Bad => write!(f, " ruim!"),
            Status::Normal => write!(f, " normal!"),
            Status::Good => write!(f, " bom!"),
            Status::Excelent => write!(f, " excelente!"),
        }
    }
}

impl Protagonist {
    pub fn hunt(&mut self, period: TimeOfDay) -> Event {
        let quality = calculate_output(period);
        let description = String::from("Você foi caçar para se alimentar pelo dia... e foi");

        let hp_status = self.hp.unwrap_or(Status::Horrible) as i8;
        let hunger_status = self.hunger.unwrap_or(Status::Horrible) as i8;
        let sleep_status = self.sleep.unwrap_or(Status::Horrible) as i8;

        self.hp = match quality {
            Some(Status::Horrible) => update_state(hp_status, -2),
            Some(Status::Bad) => update_state(hp_status, -1),
            Some(Status::Excelent) => update_state(hp_status, 1),
            _ => self.hp
        };

        self.hunger = match quality {
            Some(Status::Horrible) => update_state(hunger_status, -2),
            Some(Status::Bad) => update_state(hunger_status, -1),
            Some(Status::Normal) => update_state(hunger_status, 1),
            Some(Status::Good) => update_state(hunger_status, 2),
            Some(Status::Excelent) => update_state(hunger_status, 3),
            _ => self.hunger
        };

        self.sleep = match quality {
            Some(Status::Horrible) => update_state(sleep_status, -3),
            Some(Status::Bad) => update_state(sleep_status, -2),
            Some(Status::Excelent) => self.sleep,
            _ => update_state(sleep_status, -1)
        };

        mount_event(description, quality, self.hp.is_none())
    }

    pub fn gather_fruits(&self, period: TimeOfDay) -> Event {
        Event { description: String::from(""), quality: None, time_spent: 0, is_dead: true } //placeholder
    }

    pub fn scavenge(&self, period: TimeOfDay) -> Event {
        Event { description: String::from(""), quality: None, time_spent: 0, is_dead: true } //placeholder
    }

    pub fn treatment(&self, period: TimeOfDay) -> Event {
        Event { description: String::from(""), quality: None, time_spent: 0, is_dead: true } //placeholder
    }

    pub fn sleep(&self, period: TimeOfDay) -> Event {
        Event { description: String::from(""), quality: None, time_spent: 0, is_dead: true } //placeholder
    }

    pub fn pass_time(&self) -> Event {
        Event { description: String::from(""), quality: None, time_spent: 0, is_dead: true } //placeholder
    }
}

fn calculate_output(period: TimeOfDay) -> Option<Status> {
    let mut rng = rand::thread_rng();
    let period = period as u8;
    let period = rng.gen_range(period..period*10);

    match period {
        0..=14 => Some(Status::Horrible),
        15..=39 => Some(Status::Bad),
        40..=79 => Some(Status::Normal),
        80..=94 => Some(Status::Good),
        95..=100 => Some(Status::Excelent),
        _ => None
    }
}

fn update_state(current_state: i8, change: i8) -> Option<Status> {
    if current_state + change < 0 {
        None
    } else if current_state + change <= 5 {
        num::FromPrimitive::from_i8(current_state + change)
    } else {
        num::FromPrimitive::from_i8(5)
    }
}

fn mount_event(description: String, quality: Option<Status>, is_dead: bool) -> Event {
    let description = description + &quality.unwrap_or(Status::Horrible).to_string();
    let quality_number = quality.unwrap_or(Status::Horrible) as i8;

    let time_spent = if quality_number - 5 < 0 {
        (quality_number - 5) * -1
    } else {
        quality_number - 5
    };

    let time_spent = time_spent as u8;

    Event {
        description,
        quality, 
        time_spent, 
        is_dead
    }
}
