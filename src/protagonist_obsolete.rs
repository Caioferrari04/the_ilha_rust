use std::fmt;
use rand::Rng;
use crate::time::TimeOfDay;

pub struct Protagonist {
    pub hp: i8,
    pub hunger: i8,
    pub sleep: i8
}

pub struct Result {
    pub description: String,
    pub hp_gl: i8,
    pub hunger_gl: i8,
    pub sleep_gl: i8,
    pub time_spent: u16
}

#[derive(FromPrimitive)]
pub enum Event {
    Horrible = 0,
    Bad = 1,
    Normal = 2,
    Good = 3,
    Excelent = 4
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Event::Horrible => write!(f, " obteve resultados desastrosos!"),
            Event::Bad => write!(f, " obteve resultados ruins!"),
            Event::Normal => write!(f, " obteve resultados."),
            Event::Good => write!(f, " obteve bons resultados!"),
            Event::Excelent => write!(f, " obteve excelentes resultados!"),
        }
    }
}

impl Protagonist {
    fn trigger_event(&mut self, event_id: i8, event_description: String) -> Result {
        let event: Option<Event> = num::FromPrimitive::from_i8(event_id);
        let mut rng = rand::thread_rng();
        let hp_gl: i8;
        let hunger_gl: i8;
        let sleep_gl: i8;
        let time_spent: u16;

        match event {
            None => { panic!("Evento invalido") },
            Some(Event::Horrible) => {
                hp_gl = -rng.gen_range(20..=40);
                hunger_gl = -rng.gen_range(20..=40);
                sleep_gl = -rng.gen_range(30..=50);
                time_spent = rng.gen_range(2..=5);
            }
            Some(Event::Excelent) => {
                hp_gl = 0;
                hunger_gl = rng.gen_range(50..=100);
                sleep_gl = -rng.gen_range(0..=20);
                time_spent = 1;
            }
            _ => {
                hp_gl = -rng.gen_range(0..=10);
                hunger_gl = rng.gen_range(5..=30);
                sleep_gl = -rng.gen_range(10..=30);
                time_spent = rng.gen_range(1..=3);
            }
        };
                        
        Result { 
            description: event_description + &event.unwrap().to_string(),
            hp_gl,
            hunger_gl,
            sleep_gl,
            time_spent
        }
    }

    pub fn update_state(&mut self, result: Result) {
        self.hp = if self.hp + result.hp_gl <= 0 {
            0
        } else if self.hp + result.hp_gl <= 100 {
            self.hp + result.hp_gl
        } else {
            100
        };

        self.sleep = if self.sleep + result.sleep_gl <= 0 {
            0
        } else if result.sleep_gl + result.sleep_gl <= 100 {
            self.sleep + result.sleep_gl
        } else {
            100
        };

        self.sleep = if self.sleep + result.sleep_gl <= 0 {
            0
        } else if self.sleep + result.sleep_gl <= 100 {
            self.sleep + result.sleep_gl
        } else {
            100
        };
    }

    pub fn hunt(&mut self, time_of_day: TimeOfDay) -> Result {
        let mut rng = rand::thread_rng();
        match time_of_day {
            TimeOfDay::Madrugada => {
                self.trigger_event(rng.gen_range(0..=2), String::from("Caça durante a madrugada"))
            }
            TimeOfDay::Manha => {
                self.trigger_event(rng.gen_range(0..=3), String::from("Caça durante a manhã"))
            }
            TimeOfDay::Tarde => {
                self.trigger_event(rng.gen_range(0..=4), String::from("Caça durante a tarde"))
            }
            TimeOfDay::Noite => {
                self.trigger_event(rng.gen_range(0..=1), String::from("Caça durante a noite"))
            }
        }
    }
    
    pub fn gather_fruits(&mut self, time_of_day: TimeOfDay) -> Result {
        let mut rng = rand::thread_rng();
        match time_of_day {
            TimeOfDay::Manha | TimeOfDay::Tarde => {
                self.trigger_event(rng.gen_range(0..=3), String::from("Coleta de frutas durante o dia"))
            }
            _ => {
                self.trigger_event(rng.gen_range(0..=2), String::from("Coleta de frutas durante a noite"))
            }
        }
    }
    
    pub fn scavenge(&mut self, time_of_day: TimeOfDay) -> Result {
        let mut rng = rand::thread_rng();
        match time_of_day {
            TimeOfDay::Manha | TimeOfDay::Tarde => {
                self.trigger_event(rng.gen_range(0..=3), String::from("Procura nos escombros do avião durante o dia"))
            }
            _ => {
                self.trigger_event(rng.gen_range(0..=2), String::from("Procura nos escombros do avião durante a noite"))
            },
        }
    }
    
    pub fn treatment(&mut self, time_of_day: TimeOfDay) -> Result { //to be implemented later
        let mut rng = rand::thread_rng();
        match time_of_day {
            TimeOfDay::Manha | TimeOfDay::Tarde => {
                self.trigger_event(rng.gen_range(0..=3), String::from("Tratamento"))
            }
            _ => {
                self.trigger_event(rng.gen_range(0..=2), String::from("Tratamento"))
            },
        }
    }
    
    pub fn sleep(&mut self, time_of_day: TimeOfDay) -> Result {
        match time_of_day {
            TimeOfDay::Madrugada => {
                // self.sleep = 80;
                // self.hunger -= 20;
                // self.hp += 5;
                Result {
                    description: String::from("Dormiu por 6 horas, ainda se sente um pouco cansado"),
                    hp_gl: 5,
                    hunger_gl: -20,
                    sleep_gl: 80,
                    time_spent: 6
                }
            },
            TimeOfDay::Manha => {
                // self.sleep = 60;
                // self.hunger -= 15;
                Result {
                    description: String::from("Dormiu por 4 horas, se sente cansado"),
                    hp_gl: 0,
                    hunger_gl: -15,
                    sleep_gl: 60,
                    time_spent: 4
                }
            },
            TimeOfDay::Tarde => {
                // self.sleep += 20;
                // self.hunger -= 10;
                Result {
                    description: String::from("Cochilou por 2 horas, sente que conseguiu descansar um pouco"),
                    hp_gl: 0,
                    hunger_gl: -10,
                    sleep_gl: 20,
                    time_spent: 2
                }
            },
            TimeOfDay::Noite => {
                // self.sleep = 100;
                // self.hunger -= 25;
                // self.hp += 10;
                Result {
                    description: String::from("Dormiu por 8 horas, conseguiu descansar completamente"),
                    hp_gl: 10,
                    hunger_gl: -25,
                    sleep_gl: 100,
                    time_spent: 8
                }
            },
        }
    }
    
    pub fn pass_time(&mut self) -> Result { //prep for random events
        // self.hunger -= 5;
        // self.sleep -= 5;
        Result {
            description: String::from("Passou 1 hora sentado em seu acampamento"),
            hp_gl: 0,
            hunger_gl: -5,
            sleep_gl: -5,
            time_spent: 1
        }
    } 
}
