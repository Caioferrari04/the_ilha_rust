use crate::{time::TimeOfDay, Protagonist};
use rand::Rng;

pub fn hunt(time_of_day: TimeOfDay, protagonist: &mut Protagonist) -> u16 {
    let mut rng = rand::thread_rng();
    match time_of_day {
        TimeOfDay::Madrugada => {
            protagonist.hp -= rng.gen_range(0..40);
            protagonist.hunger += rng.gen_range(0..20);
            protagonist.sleep -= 40;
            4
        },
        TimeOfDay::Manha => {
            protagonist.hp -= rng.gen_range(0..10);
            protagonist.hunger += rng.gen_range(5..20);
            protagonist.sleep -= 20;
            2
        },
        TimeOfDay::Tarde => {
            protagonist.hp -= rng.gen_range(0..5);
            protagonist.hunger += rng.gen_range(10..20);
            protagonist.sleep -= 10;
            1
        },
        TimeOfDay::Noite => {
            protagonist.hp -= rng.gen_range(0..50);
            protagonist.hunger += rng.gen_range(0..10);
            protagonist.sleep -= 50;
            5
        }
    }
}

pub fn gather_fruits(time_of_day: TimeOfDay, protagonist: &mut Protagonist) -> u16 {
    match time_of_day {
        TimeOfDay::Madrugada => 2,
        TimeOfDay::Manha => 1,
        TimeOfDay::Tarde => 1,
        TimeOfDay::Noite => 2
    }
}

pub fn scavenge(time_of_day: TimeOfDay, protagonist: &mut Protagonist) -> u16 {
    match time_of_day {
        TimeOfDay::Madrugada => 2,
        TimeOfDay::Manha => 1,
        TimeOfDay::Tarde => 1,
        TimeOfDay::Noite => 2
    }
}

pub fn treatment(time_of_day: TimeOfDay, protagonist: &mut Protagonist) -> u16 {
    match time_of_day {
        TimeOfDay::Madrugada => 2,
        TimeOfDay::Manha => 1,
        TimeOfDay::Tarde => 1,
        TimeOfDay::Noite => 2
    }
}

pub fn sleep(time_of_day: TimeOfDay, protagonist: &mut Protagonist) -> u16 {
    match time_of_day {
        TimeOfDay::Madrugada => 6,
        TimeOfDay::Manha => 4,
        TimeOfDay::Tarde => 2,
        TimeOfDay::Noite => 8
    }
}
