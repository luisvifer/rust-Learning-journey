/*
 * Escribe un programa que, dado un número, compruebe y muestre si es primo, fibonacci y par.
 * Ejemplos:
 * - Con el número 2, nos dirá: "2 es primo, fibonacci y es par"
 * - Con el número 7, nos dirá: "7 es primo, no es fibonacci y es impar"
 */
use std::io;
fn main() {
    let num: u128;
    println!();
    loop {
        match leer_numero("Dame un número") {
           Ok(current) =>{ num = current;
                   break;
               },
           Err(e) => println!("Error: {}", e),
       }
   }
    let prime = prime_str(num);
    let fibo = fibo_str(num);
    let even = even_str(num);

    println!("El número {} {}, {} y {}", num, prime, fibo, even);
   
}
fn is_prime(num: u128) -> bool {
    if num <= 1 {
        return false; // 0 y 1 no son primos
    }
    if num == 2 {
        return true; // 2 es el único número primo par
    }
    if num % 2 == 0 {
        return false; // Descartar números pares mayores que 2
    }

    let raiz = (num as f64).sqrt() as u128;
    for i in (3..=raiz).step_by(2) {
        if num % i == 0 {
            return false;
        }
    }

    true
}

fn prime_str(num: u128) -> String {
    if is_prime(num) {
        String::from("es primo")
    } else {
        String::from("no es primo")
    }
}

fn is_fibonacci(num:u128) -> bool{
    is_fibonacci_with_a_b(num, 0, 1)
}
fn is_fibonacci_with_a_b(num: u128, a:u128, b:u128) -> bool {
    let next_fibo = a + b;
    if next_fibo == num || a == num || b == num {
        return true;
    }else if next_fibo >  num  {
        return false;
    }
    is_fibonacci_with_a_b(num, b, next_fibo)

}
    

fn fibo_str(num: u128) -> String {
    if is_fibonacci(num){
        String::from("fibonacci")
    }
    else {
        String::from("no es fibonacci")
    }
}

fn even_str(num: u128) -> String {
    if is_even(num) {
        String::from("es par")
    } else {
        String::from("es impar")
    }
}

fn is_even(num: u128) -> bool {
    if num % 2 == 0 {
        return true;
    }
    false
}




// Función para leer y validar el número
fn leer_numero(msg: &str) -> Result<u128, String> {
    let mut input = String::new();

    println!("{}", msg);

    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la línea");

    let num: u128 = input.trim().parse().map_err(|_| "Por favor, introduce un número válido.")?;

    // Verifica si el número está en el rango deseado
   
        Ok(num)
   
}



