
use std::io;
mod my_hash_table;

use my_hash_table::Hashable;

pub use self::my_hash_table::MyHashTable;



pub struct Pool {
    voted: MyHashTable<String,bool>,
}

trait Voted {
   fn check_voter(&mut self,name:String)-> Result<String,String>;
}

impl Hashable for String {
    fn hash(&self) -> usize {
        self.len() * self.as_bytes().len() * 122000
    }
}

impl Voted for Pool {
     fn check_voter(&mut self, name:String) -> Result<String,String>{
    
        match self.voted.find(&name) {
            Some(&true) => Err(format!("Kick them out {}", name)),
            Some(&false) => {
                self.voted.put(name.clone(), true);
                Ok(format!("let them vote! {}", name))
            },
            None => {
                self.voted.insert(name.clone(), true);
                Ok(format!("let them vote! {}", name))
            }
        }
       
    }
    
}

impl Pool {
    pub fn new() -> Pool{
        Pool {
            voted : MyHashTable::default()
        }
    }
}



fn main() {
     let mut pool = Pool::new();
      (0.. 10).for_each(|_| {
        match read_name("Give me a new voter:") {
            Ok(voter) => match pool.check_voter(voter){
                Ok(value) =>  println!("{}", value),
                Err(value) =>  println!("{}", value),
            },
            Err(e) => println!("Error: {}", e),
        }
      });

}

// Función para leer y validar un valor booleano
fn read_name(msg: &str) ->  Result<String,String> {
    let mut input = String::new();

    println!("{}", msg);

    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_lowercase()), // Si la lectura es exitosa
        Err(_) => Err("Error al leer la línea".to_string()), // En caso de error
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_voter() {
        let mut pool = Pool::new(); // Replace with your actual constructor

        let voter_name = "Alice".to_string();

        // Scenario 1: Voter has not voted yet
        assert_eq!(pool.check_voter(voter_name.clone()), Ok("let them vote! Alice".to_string()));

        // Scenario 2: Voter tries to vote again
        assert_eq!(pool.check_voter(voter_name.clone()), Err("Kick them out Alice".to_string()));

        let another_voter = "Bob".to_string();

        // Scenario 3: A different voter who has not voted yet
        assert_eq!(pool.check_voter(another_voter), Ok("let them vote! Bob".to_string()));
    }
}


