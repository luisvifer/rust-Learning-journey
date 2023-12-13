/* Reto 3
 * Escribe un programa que sea capaz de generar contraseñas de forma aleatoria.
 * Podrás configurar generar contraseñas con los siguientes parámetros:
 * - Longitud: Entre 8 y 16.
 * - Con o sin letras mayúsculas.
 * - Con o sin números.
 * - Con o sin símbolos.
 * (Pudiendo combinar todos estos parámetros entre ellos)
 */


use std::io;
use rand::{Rng, seq::SliceRandom};

#[derive(Clone, Copy)]
 struct PassPercent{
    normal: i32,
    capital: i32,
    number: i32,
    symbol: i32,
 }

 
fn main() {
    let  longitud:i32;
    let  with_capital: bool;
    let  with_numbers: bool;
    let  with_symbols: bool;
   loop {
         match leer_numero("Por favor, introduce la longitud de la password un número entre 8 y 16:") {
            Ok(num) =>{ longitud = num;
                    break;
                },
            Err(e) => println!("Error: {}", e),
        }
    }

    loop {
        match leer_booleano("Con o sin letras mayúsculas.(Si/No):") {
           Ok(value) =>{ with_capital = value;
                   break;
               },
           
           Err(e) => println!("Error: {}", e),
       }
   }
   loop {
    match leer_booleano("Con o sin números..(Si/No):") {
       Ok(value) =>{ with_numbers = value;
               break;
           },
       Err(e) => println!("Error: {}", e),
   }
}
loop {
    match leer_booleano("Con o sin símbolos.(Si/No):") {
       Ok(value) =>{ with_symbols = value;
               break;
           },
       Err(e) => println!("Error: {}", e),
   }
}
println!("Longitud {}, con mayusculas {}, con numeros {} , con simbolos {}", longitud,with_capital,with_numbers,with_symbols);
let  mut capital_len = 0;
let  mut numbers_len = 0;
let  mut symbol_len = 0;

if with_capital {
    capital_len = (longitud as f64 * 0.2).ceil() as i32;
}

if with_numbers {
    numbers_len = (longitud as f64 * 0.2).ceil() as i32;
}
if with_symbols {
    symbol_len = (longitud as f64 * 0.2).ceil() as i32;
}
let normal = longitud - capital_len - numbers_len - symbol_len;
let percent= PassPercent{
    normal:normal,
    capital: capital_len,
    number: numbers_len,
    symbol: symbol_len

};
println!("{}",generate_pass(percent));

    
}

// Función para leer y validar el número
fn leer_numero(msg: &str) -> Result<i32, String> {
    let mut input = String::new();

    println!("{}", msg);

    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la línea");

    let num: i32 = input.trim().parse().map_err(|_| "Por favor, introduce un número válido.")?;

    // Verifica si el número está en el rango deseado
    if num >= 8 && num <= 16 {
        Ok(num)
    } else {
        Err(format!("El número debe estar entre 8 y 16. Introducido: {}", num))
    }
}

// Función para leer y validar un valor booleano
fn leer_booleano(msg: &str) -> Result<bool, String> {
    let mut input = String::new();

    println!("{}", msg);

    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la línea");

    match input.trim().to_lowercase().as_str() {
        "si" => Ok(true),
        "no" => Ok(false),
        value  => Err(format!("Entrada no válida {value}").to_string()),
    }
}

fn generate_pass(percent: PassPercent) -> String {
    let letter = (b'a'..=b'z')   // Rango de bytes desde 'a' hasta 'z'
        .map(|c| c as char)          // Convertir cada byte a char
        .collect::<Vec<_>>();   
    let capital_letter = (b'A'..=b'Z')   // Rango de bytes desde 'a' hasta 'z'
        .map(|c| c as char)          // Convertir cada byte a char
        .collect::<Vec<_>>(); 
    let numbers  = (b'0'..=b'9')
        .map(|c| c as char)          // Convertir cada byte a char
        .collect::<Vec<_>>(); 
    let mut symbols = Vec::new();

    // Agrega los símbolos de cada rango a un vector
    symbols.extend((33u8..=47u8).map(|c| c  as char));
    symbols.extend((58u8..=64u8).map(|c| c as char));
    symbols.extend((91u8..=96u8).map(|c| c as char));
    symbols.extend((123u8..=126u8).map(|c| c as char));
    let mut pass = Vec::new();
    pass.extend(generate_rand_vector_char(percent.normal,letter));
    pass.extend(generate_rand_vector_char(percent.capital,capital_letter));
    pass.extend(generate_rand_vector_char(percent.number,numbers));
    pass.extend(generate_rand_vector_char(percent.symbol,symbols));
    let mut rng = rand::thread_rng();
    pass.shuffle(&mut rng);
    return pass.iter().collect();

}

fn generate_rand_vector_char(number:i32,vector: Vec<char>) -> Vec<char> {
   
    let mut rng = rand::thread_rng();
    let elementos: Vec<char> = (0..number)
    .map(| _ | vector[rng.gen_range(0..vector.len())])
    .collect();
    elementos
    
}