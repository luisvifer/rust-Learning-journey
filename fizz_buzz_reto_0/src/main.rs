/* RETO 0
 * Escribe un programa que muestre por consola (con un print) los
 * números de 1 a 100 (ambos incluidos y con un salto de línea entre
 * cada impresión), sustituyendo los siguientes:
 * - Múltiplos de 3 por la palabra "fizz".
 * - Múltiplos de 5 por la palabra "buzz".
 * - Múltiplos de 3 y de 5 a la vez por la palabra "fizzbuzz".
 */

fn main() {
    println!("Hello, world!");
    for i in 1..=100 {
        println!("{}",fizz_buzz(i));
    }
    for i in 1..=100 {
        println!("{}",fizz_buzz_pm(i));
    }
}


fn fizz_buzz(num:u32) -> String {
    
    if num % 15 == 0 {
       return String::from("fizzbuzz");
    }else if num % 5 == 0 {
        return  String::from("buzz");
    } else if num % 3 == 0 {
        return String::from("fizz");
    }
    num.to_string() 
} 

fn fizz_buzz_pm(num:u32 ) -> String{
    return match (num % 3, num % 5){
        (0,0) => String::from("fizzbuzz"), 
        (_,0) => String::from("fizz"), 
        (0,_) => String::from("buzz"), 
         _ =>   num.to_string(),
    };
}