/*
 * Crea un programa que calcule quien gana más partidas al piedra,
 * papel, tijera, lagarto, spock.
 * - El resultado puede ser: "Player 1", "Player 2", "Tie" (empate)
 * - La función recibe un listado que contiene pares, representando cada jugada.
 * - El par puede contener combinaciones de "🗿" (piedra), "📄" (papel),
 *   "✂️" (tijera), "🦎" (lagarto) o "🖖" (spock).
 * - Ejemplo. Entrada: [("🗿","✂️"), ("✂️","🗿"), ("📄","✂️")]. Resultado: "Player 2".
 * - Debes buscar información sobre cómo se juega con estas 5 posibilidades.
 */

  enum Winner {
    Player1, Player2, Tie
  }
 

fn main() {
   let games = [("🗿","✂️"), ("✂️","🗿"), ("📄","✂️")];
    let mut player1:i32 = 0;
    let mut player2:i32 = 0;
    let mut tie :i32 = 0;
    for i in 0..games.len() {
        let result = game(games[i]);
        match result {
            Winner::Player1 => player1 = player1 +1,
            Winner::Player2 => player2 = player2 +1,
            Winner::Tie => tie = tie +1,

        }
    } 
    if player1 > player2 {
        if player1 > tie {
            println!("Resultado \"Player 1\"")
        }else {
            println!("Resultado \"Tie\"")
        }
    }else  if player2 > tie{
        println!("Resultado \"Player 2\"")

    }else {
        println!("Resultado \"Tie\"")
    }
}


fn game(current_game: (&str, &str)) -> Winner {
    match current_game {
        ("🗿", "✂️") => Winner::Player1,
        ("🗿", "🦎") =>  Winner::Player1,
        ("🗿", "🖖") =>  Winner::Player2,
        ("🗿", "📄") =>  Winner::Player2,
        ("📄", "✂️") =>  Winner::Player2,
        ("📄", "🦎") =>  Winner::Player2,
        ("📄", "🖖") =>  Winner::Player1,
        ("📄", "🗿") =>  Winner::Player1,
        ("✂️","🗿") =>  Winner::Player2,
        ("✂️","🦎") =>  Winner::Player1,
        ("✂️","🖖") =>  Winner::Player2,
        ("✂️","📄") =>  Winner::Player1,
        ("🖖","📄") =>  Winner::Player2,
        ("🖖","🗿") =>  Winner::Player1,
        ("🖖","🦎") =>  Winner::Player2,
        ("🖖","✂️") =>  Winner::Player1,
        ("🦎","📄") =>  Winner::Player1,
        ("🦎","🗿") =>  Winner::Player2,
        ("🦎","🖖") =>  Winner::Player1,
        ("🦎","✂️") =>  Winner::Player2,
        ("🗿","🗿") => Winner::Tie,
        ("✂️","✂️") => Winner::Tie,
        ("🖖","🖖") => Winner::Tie,
        ("🦎","🦎") => Winner::Tie,
        ("📄","📄") => Winner::Tie,
        (_, _) => Winner::Tie,
    }
}