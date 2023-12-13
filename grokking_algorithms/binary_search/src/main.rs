
use std::*;

fn main() {
    let number:i32;
    let data:Vec<i32> =(0..=100).collect();
    let low: usize = 0;
    let high: usize = data.len();
    loop {
        match leer_numero("Dame un número") {
           Ok(current) =>{ number = current;
                   break;
               },
           Err(e) => println!("Error: {}", e),
       }
   }
   println!("Búsqueda binaria recursiva");
    match binary_search_recursive(&data, number, low, high){
        Ok(current) =>{ println!("El numero está en la posición : {}", current)
        },
    Err(e) => println!("Error: {}", e),
    }

    println!("Búsqueda binaria");
    match binary_search(&data, number){
        Ok(current) =>{ println!("El numero está en la posición : {}", current)
        },
    Err(e) => println!("Error: {}", e),
    }
   

   
        
    
}
fn leer_numero(msg: &str) -> Result<i32, String> {
    let mut input = String::new();

    println!("{}", msg);

    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la línea");

    let num: i32 = input.trim().parse().map_err(|_| "Por favor, introduce un número válido.")?;

    // Verifica si el número está en el rango deseado
   
        Ok(num)
   
}

fn binary_search_recursive (data:&Vec<i32>, number: i32, low : usize, high: usize) -> Result<usize,String>{
    
    if low > high {
        return Err(format!("No se ha encontrado {}", number.to_string()));
    }

    //let middle: usize = (low +  high) / 2; // no evita el desbordamiento de la pila
    let middle: usize = low + (high - low) / 2; // evitamos el desbordamiento de la pila
    println!("Comprobando posición : {}", middle);
    match data.get(middle) {
        Some(&guess) if guess == number => Ok(middle),
        Some(&guess) if guess < number => binary_search_recursive(data, number, middle + 1, high),
        Some(&guess) if guess > number => binary_search_recursive(data, number, low, middle - 1),
        Some(_) => unreachable!(), // Esta rama no debería alcanzarse nunca
        None => Err(format!("Índice fuera de rango")), // Esto podría suceder si el índice medio calculado está fuera de rango
    }
}

fn binary_search (data:&Vec<i32>, number:i32) -> Result<usize,String>{
    let mut low: usize = 0;
    let mut high:usize = data.len();
    while low <= high {
        let middle = low + (high - low ) / 2;
        println!("Comprobando posición : {}", middle);
        match data.get(middle){
            Some(&guess) if guess == number => return Ok(middle),
            Some(&guess) if guess < number =>  low = middle + 1,
            Some(&guess) if guess > number => high = middle - 1,
            Some(_) => unreachable!(),
            None => return Err(format!("Índice fuera de rango")),
        }
    }
     Err(format!("No se ha encontrado {}", number.to_string()))

}
