/* Reto 1
 * Escribe un programa que reciba un texto y transforme lenguaje natural a
 * "lenguaje hacker" (conocido realmente como "leet" o "1337"). Este lenguaje
 *  se caracteriza por sustituir caracteres alfanuméricos.
 * - Utiliza esta tabla (https://www.gamehouse.com/blog/leet-speak-cheat-sheet/) 
 *   con el alfabeto y los números en "leet".
 *   (Usa la primera opción de cada transformación. Por ejemplo "4" para la "a")
 */

use std::io;

fn main() {
    println!("Give me a comment to leet!");
    let mut text = String::new();
    io::stdin()
    .read_line(&mut text)
    .expect("Failed to read line");
    let text_to_leet: String = text.trim().chars().map(|c| {
        leet(c)
    }).collect();

    println!("{}",text_to_leet);
}


fn leet(character:char) -> String{

   let charater_to_upper: char = character.to_ascii_uppercase();
    match charater_to_upper { 
        'A' => String::from("4"),
        'B' => String::from("I3"),
        'C' => String::from("["),
        'D' => String::from(")"),
        'E' => String::from("3"),
        'F' => String::from("|="),
        'G' => String::from("&"),
        'H' => String::from("#"),
        'I' => String::from("1"),
        'J' => String::from(",_|"),
        'K' => String::from(">|"),
        'L' => String::from("1"),
        'M' => String::from("/\\/\\"),
        'N' => String::from("^/"),
        'O' => String::from("0"),
        'P' => String::from("|*"),
        'Q' => String::from("(_,)"),
        'R' => String::from("I2"),
        'S' => String::from("5"),
        'T' => String::from("7"),
        'U' => String::from("(_)"),
        'V' => String::from("\\/"),
        'W' => String::from("\\/\\/"),
        'X' => String::from("><"),
        'Y' => String::from("j"),
        'Z' => String::from("2"),
        '0' => String::from("o"),
        '1' => String::from("L"),
        '2' => String::from("R"),
        '3' => String::from("E"),
        '4' => String::from("A"),
        '5' => String::from("S"),
        '6' => String::from("b"),
        '7' => String::from("T"),
        '8' => String::from("B"),
        '9' => String::from("g"), 
         _ => character.to_string()
    }
} 