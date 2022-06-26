extern crate num;
#[macro_use]
extern crate num_derive;

mod time;
mod protagonist;

use std::io::{self, Write};
use protagonist::{Protagonist, Status};

fn main() {
    let repeat_minus: String = str::repeat("-=", 60);

    println!("{}", repeat_minus);
    println!("
        Bem vindo ao The Ilha;
    ");
    println!("{}", repeat_minus);
    pause_display();

    println!("{}", repeat_minus);
    println!("
        Você está fazendo uma viagem de negócios, num jatinho fretado por sua empresa.
        Porém, no meio do trajeto, uma tempestade horrivel se forma e acaba derrubando seu avião, 
        que cai em uma ilha no meio do oceano.
    ");
    println!("{}", repeat_minus);
    pause_display();
    
    println!("{}", repeat_minus);
    println!("
        Por algum milagre, você sobrevive a queda (o piloto não teve a mesma sorte), e agora,
        você deverá sobreviver por 7 dias nessa ilha, que é o tempo necessário para sua empresa 
        rastrear o local da queda e enviar o resgate.
    ");
    println!("{}", repeat_minus);
    pause_display();

    game(repeat_minus);
}

fn game(repeat_minus: String) {
    'outer: loop {
        let mut protagonist = Protagonist { 
            hp: Some(Status::Excelent), 
            hunger: Some(Status::Excelent), 
            sleep: Some(Status::Excelent) 
        };
    
        println!("{}", repeat_minus);
        println!("
            Cace, ou procure por frutas para sobreviver. 
            Boa sorte, e tente não morrer!
        ");
        println!("{}", repeat_minus);
    
        let mut time_passed: i8 = 12;              
        let option_menu: &str = "Opções:
        [1] - Caçar/Pescar
        [2] - Colher frutas
        [3] - Vasculhar escombros do avião
        [4] - Tratamento
        [5] - Dormir
        [6] - Passar tempo
        [7] - Sair do Jogo\n";
    
        'inner: loop {
            pause_display();
            time_passed = time::process_time(time_passed);
            let time_of_day = time::time_of_day(&time_passed);
    
            println!("Status: \n   Horas: {}\n   Saude: Você se sente{}\n   Fome: Você se sente{}\n   Cansaço: Você se sente{}", 
            time::to_string(&time_passed) + ", " + &time_of_day.to_string(),
            protagonist.hp.unwrap_or(Status::Horrible),
            protagonist.hunger.unwrap_or(Status::Horrible),
            protagonist.sleep.unwrap_or(Status::Horrible));
    
            println!("{}", option_menu);
            print!("   >");
    
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
                _ => break 'outer
            };
    
            time_passed += result_action.time_spent;
    
            println!("Resultados: \n    {},\n    Tempo gasto: {}", 
            result_action.description,
            result_action.time_spent);
    
            println!("{}", repeat_minus);
    
            if result_action.is_dead {
                println!("Você morreu! Deseja tentar novamente? [s/n]");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Valor inválido!");
    
                let input = input.trim().to_lowercase();
    
                if input == "s" {
                    break 'inner;
                }
                break 'outer;
            }
        }
    }

    println!("Até mais!");
}

fn pause_display() {
    println!("Pressione qualquer tecla para continuar;");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("houve um erro");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
