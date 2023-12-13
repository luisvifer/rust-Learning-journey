/*
 * Crea un generador de números pseudoaleatorios entre 0 y 100.
 * - No puedes usar ninguna función "random" (o semejante) del lenguaje de programación seleccionado.
 *
 * Es más complicado de lo que parece...
 */
use std::time::{SystemTime, UNIX_EPOCH};
fn main() {
    println!("Genera un número aleatorio entre 0 y 100!");
    for _i in 0 ..100 {
        print!("{},",rand());
    }
    println!();

    println!("Second version:Genera un número aleatorio entre 0 y 100!");
    let mut rand = Rand::new();

    for _i in 0 ..100 {
        print!("{},",rand.next());
    }
    println!();

}


fn rand ()-> i32 {
    let mut random_number:i32 = 0;
    
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => {
            // Obtener los milisegundos
            let nanos = n.as_nanos();
           
            let seed = nanos % 65535 ;
            random_number = ((nanos * seed)  % 101) as i32;
          
        }
        Err(_) => println!("Error en el calculo del valor aleatorio"),
    }
    random_number
    

} 

struct Rand {
    seed:u128
}

trait  INext{
    fn next(&mut self) -> i32;
}

impl Rand {
    pub fn new () -> Self {
        let new_seed =  match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(n) => {
                    // Obtener los milisegundos
                    let nanos = n.as_nanos();
                   
                    let seed = nanos % 65535 ;
                    seed
                }
                Err(_) => 5555u128,
        };
        Self { seed: new_seed, }
}

}

impl INext for Rand {
    fn next(&mut self)  -> i32 {
        let mut random_number:i32 = 0;
    
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => {
                // Obtener los milisegundos
                let nanos = n.as_nanos();
               
                let seed = nanos % 65535 ;
                self.seed = self.seed + seed;
                random_number = ((nanos + self.seed)  % 101) as i32;
              
            }
            Err(_) => println!("Error en el calculo del valor aleatorio"),
        }
        random_number 
    }
}