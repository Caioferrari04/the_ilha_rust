extern crate num;
#[macro_use]
extern crate num_derive;

mod time;
mod protagonist;

use std::{thread, time::Duration, io::{self, Write}};
use protagonist::Protagonist;

fn main() {
    let repeat_minus: String = str::repeat("-=", 60);
    let sleep_time: Duration = Duration::from_secs(3);

    println!("{}", repeat_minus);
    println!("
        Bem vindo ao The Ilha;
    ");
    println!("{}", repeat_minus);
    thread::sleep(sleep_time);

    println!("
        Você está fazendo uma viagem de negócios, num jatinho fretado por sua empresa.
        Porém, no meio do trajeto, uma tempestade horrivel se forma e acaba derrubando seu avião, 
        que cai em uma ilha no meio do oceano.
    ");
    thread::sleep(sleep_time);

    println!("
        Por algum milagre, você sobrevive a queda (o piloto não teve a mesma sorte), e agora,
        você deverá sobreviver por 7 dias nessa ilha, que é o tempo necessário para sua empresa 
        rastrear o local da queda e enviar o resgate.
    ");
    thread::sleep(sleep_time);

    println!("
        Cace, ou procure por frutas para sobreviver. 
        Boa sorte, e tente não morrer!
    ");
    
    let mut protagonist: Protagonist = Protagonist { hp: 100, hunger: 100, sleep: 100 };
    let mut time_passed: u16 = 12;              
    let option_menu: &str = "
        Opções:
            [1] - Caçar/Pescar
            [2] - Colher frutas
            [3] - Vasculhar escombros do avião
            [4] - Tratamento
            [5] - Dormir
            [6] - Passar tempo
            [7] - Sair do Jogo\n";

    loop {
        time_passed = time::process_time(time_passed);
        let time_of_day: time::TimeOfDay = time::time_of_day(&time_passed);

        println!("
        Status:
            Horas: {}
            Fome: {}
            Saude: {}
            Sono: {}
        ", 
        time::to_string(&time_passed) + ", " + &time_of_day.to_string(),
        protagonist.hunger,
        protagonist.hp,
        protagonist.sleep
        );

        println!("{}", option_menu);
        print!("
        --->");

        io::stdout().flush().expect("houve um erro");

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Valor inválido!");

        let input = input.trim().parse::<i32>().expect("NaN");

        let result_action = match input {
            1 => protagonist.hunt(time_of_day),
            2 => protagonist.gather_fruits(time_of_day),
            3 => protagonist.scavenge(time_of_day),
            4 => protagonist.treatment(time_of_day),
            5 => protagonist.sleep(time_of_day),
            6 => protagonist.pass_time(),
            _ => break
        };

        time_passed += result_action.time_spent;

        println!("
        Resultados:
        {},
        Ganho/perda de saude: {},
        Ganho/perda de fome: {},
        Ganho/perda de cansaço: {},
        Tempo gasto: {} 
        ", 
        result_action.description,
        result_action.hp_gl,
        result_action.hunger_gl,
        result_action.sleep_gl,
        result_action.time_spent
        );
    }

    println!("
    Até mais!
    ");
}
