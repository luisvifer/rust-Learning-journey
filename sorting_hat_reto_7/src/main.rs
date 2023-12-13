/*
 * Crea un programa que simule el comportamiento del sombrero seleccionador del
 * universo mágico de Harry Potter.
 * - De ser posible realizará 5 preguntas (como mínimo) a través de la terminal.
 * - Cada pregunta tendrá 4 respuestas posibles (también a selecciona una a través de terminal).
 * - En función de las respuestas a las 5 preguntas deberás diseñar un algoritmo que
 *   coloque al alumno en una de las 4 casas de Hogwarts (Gryffindor, Slytherin , Hufflepuff y Ravenclaw)
 * - Ten en cuenta los rasgos de cada casa para hacer las preguntas y crear el algoritmo seleccionador.
 *   Por ejemplo, en Slytherin se premia la ambición y la astucia.
 */

use std::io;

enum AnswerChoice {
    Gryffindor,
    Slytherin,
    Hufflepuff,
    Ravenclaw,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let questions= [
        "Cual es tu color favorito?\n a) rojo\n b) verde\n c) amarillo\n d) azul\n",
        "Cual es tu animal favorito?\n a) leon\n b) serpiente\n c) mapache\n d) cuervo\n",
        "Como te defines?\n a) Fiel y honesto\n b) Astuto y ambicioso\n c) Pfff\n d) Orgulloso e inteligente\n"];
    let mut answer =[0u16;4];
    for i in 0..questions.len() {
        loop {
             match  input_question(questions[i]) {
                 Ok(choice) => {
                    match choice {
                        AnswerChoice::Gryffindor => {
                            answer[0] = answer[0] + 5;
                            answer[1] = answer[1] + 1;
                            answer[2] = answer[2] + 3;
                            answer[3] = answer[3] + 2;

                        },
                        AnswerChoice::Slytherin => {
                            answer[0] = answer[0] + 1;
                            answer[1] = answer[1] + 5;
                            answer[2] = answer[2] + 2;
                            answer[3] = answer[3] + 3;
                        },
                        AnswerChoice::Hufflepuff => {
                            answer[0] = answer[0] + 3;
                            answer[1] = answer[1] + 2;
                            answer[2] = answer[2] + 5;
                            answer[3] = answer[3] + 1;  
                        },
                        AnswerChoice::Ravenclaw => {
                            answer[0] = answer[0] + 2;
                            answer[1] = answer[1] + 3;
                            answer[2] = answer[2] + 1;
                            answer[3] = answer[3] + 5;  
                        },
                    }
                    break;
                },

                Err(e)=> {
                    println!("Error: {}", e)
                },
            }
        }
    }
    let mut choice_index=0;
    for j in 0 ..answer.len() {
        if answer[j] > answer[choice_index] {
            choice_index = j;
        }
    }

    match choice_index {
        0 => println!("Tu casa es Gryffindor"),
        1 => println!("Tu casa es Slytherin"),
        2 => println!("Tu casa es Hufflepuff"),
        3 => println!("Tu casa es Ravenclaw"),
        value =>  return Err(format!("Tu casa no esválida {value}").to_string().into()),
    }
    Ok(())
}


fn input_question(question: &str) -> Result<AnswerChoice, String> {
    let mut input = String::new();
    println!("{}", question);
    io::stdin()
        .read_line(&mut input)
        .expect("Error leyendo la línea");
    match input.trim().to_lowercase().as_str() {
        "a" => Ok(AnswerChoice::Gryffindor),
        "b" => Ok(AnswerChoice::Slytherin),
        "c" => Ok(AnswerChoice::Hufflepuff),
        "d" => Ok(AnswerChoice::Ravenclaw),
        value => Err(format!("Entrada no válida {value}").to_string()),
    }
}
