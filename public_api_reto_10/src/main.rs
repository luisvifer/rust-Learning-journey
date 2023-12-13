/* Reto 10
 * Llamar a una API es una de las tareas más comunes en programación.
 *
 * Implementa una llamada HTTP a una API (la que tú quieras) y muestra su
 * resultado a través de la terminal. Por ejemplo: Pokémon, Marvel...
 *
 * Aquí tienes un listado de posibles APIs: 
 * https://github.com/public-apis/public-apis
 */
use serde::{Deserialize, Serialize};
use reqwest;
use rand::Rng;


#[derive(Deserialize, Serialize, Debug)]
struct Response {
    facts: Vec<String>,
    success: bool,
}



#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let rand_number = rand::thread_rng().gen_range(1..=100);
    let url = format!("http://dog-api.kinduff.com/api/facts?number={}",rand_number);

    let resp = reqwest::get(url)
        .await?
        .json::<Response>()
        .await?;

    println!("Let's show {} dog facts",rand_number);
    println!("{:#?}", resp);
    Ok(())
}
