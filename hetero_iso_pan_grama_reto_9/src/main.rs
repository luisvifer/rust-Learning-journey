use std::io;
use std::collections::HashMap;

fn main() {
    let sentence ;
    loop {
        match read_sentence("Introduce una frase:") {
           Ok(value) =>{ sentence = value;
                   break;
               },
           Err(e) => println!("Error: {}", e),
       }
    }
    println!("La frase {sentence} es:" );
    if is_heterogram(sentence.as_str()){
        println!("Es heterograma")
    } else {
        println!("No es heterograma")
    }
    if is_isogram(sentence.as_str()){
        println!("Es isograma")
    }
    else {
        println!("No es isograma")
    }
    if is_pangram(sentence.as_str()){
        println!("Es panagrama")
    }
    else {
        println!("No es panagrama")
    }

}

fn is_heterogram (sentence: &str ) -> bool{
    let map = text_counter(sentence);
   
    for data in map.values() {
            if *data > 1 {
                return false;
            } 
    }
    true
}

fn is_isogram (sentence: &str ) -> bool{

    let map = text_counter(sentence);
   
    for data in map.values() {
            if *data > 1 && *data %2 != 0 {
                return false;
            } 
    }
    true
}

fn is_pangram (sentence: &str ) -> bool{
    let map = text_counter(sentence);
   
    map.keys().len() >= 27
}


fn read_sentence(msg: &str) -> Result<String,String> {
    let mut input = String::new();
    println!("{}", msg);
    
   return  match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if input.trim().to_lowercase().chars().any(|c| !c.is_ascii_alphabetic() && !c.is_whitespace() && c !='ñ') {
                Err("La entrada no debe contener dígitos, tildes ó otros caracteres especiales.".to_string())
            } else {
                Ok(input.trim().to_string())
            }  
    }
        Err(error) => Err(format!("error: {error}")),
    };
}

fn text_counter(text:&str) -> HashMap<char,i32>{
    let mut word_map:HashMap<char, i32> = HashMap::new();
     for character in text.chars().filter(|c| !c.is_whitespace()){
        let count = word_map.entry(character).or_insert(0);
        *count += 1;
    }
    word_map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_heterogram() {
        assert_eq!(is_heterogram("yuxtaponer"), true);
        assert_eq!(is_heterogram("centrifugado"), true);
        assert_eq!(is_heterogram("alado"), false);
    }

    #[test]
    fn test_is_isogram() {
        assert_eq!(is_isogram("acondicionar"), true);
        assert_eq!(is_isogram("acondicionado"), false);
    }

    #[test]
    fn test_is_pangram() {
        assert_eq!(is_pangram("Un jugoso zumo de piña y kiwi bien frio es exquisito y no lleva alcohol"), true);
        assert_eq!(is_pangram("Un jugoso zumo de piña bien frio es exquisito y no trae alcohol"), false);
    }

}






