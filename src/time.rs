pub fn process_time(&current_time: &u16) -> String {
    let processed_time: u16 = if current_time > 24 {
        current_time - 24
    } else {
        current_time
    };

    let time_of_day: &str = match processed_time {
        1..=6 => "período da madrugada",
        7..=12 => "período da manhã",
        13..=18 => "período da tarde",
        19..=24 => "período da noite",
        _ => "período da madrugada"
    };

    processed_time.to_string() + if processed_time > 1 { " horas, " } else { " hora, " } + time_of_day
}