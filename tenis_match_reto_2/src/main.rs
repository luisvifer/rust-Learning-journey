/* Reto 2
 * Escribe un programa que muestre cómo transcurre un juego de tenis y quién lo ha ganado.
 * El programa recibirá una secuencia formada por "P1" (Player 1) o "P2" (Player 2), según quien
 * gane cada punto del juego.
 *
 * - Las puntuaciones de un juego son "Love" (cero), 15, 30, 40, "Deuce" (empate), ventaja.
 * - Ante la secuencia [P1, P1, P2, P2, P1, P2, P1, P1], el programa mostraría lo siguiente:
 *   15 - Love
 *   30 - Love
 *   30 - 15
 *   30 - 30
 *   40 - 30
 *   Deuce
 *   Ventaja P1
 *   Ha ganado el P1
 * - Si quieres, puedes controlar errores en la entrada de datos.
 * - Consulta las reglas del juego si tienes dudas sobre el sistema de puntos.
 */

use std::fmt;

#[derive(Clone, Copy)]
enum PlayerName {
    P1,
    P2,
}
impl fmt::Display for PlayerName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PlayerName::P1 => "P1",
                PlayerName::P2 => "P2",
            }
        )
    }
}

#[derive(Clone, Copy)]
struct Player {
    name: PlayerName,
    point: i32,
}
impl Player {
    // Constructor para Player
    fn new(name: PlayerName, point: i32) -> Player {
        Player { name, point }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let state_str = match self.point {
            0 => "Love".to_string(),
            1 => "15".to_string(),
            2 => "30".to_string(),
            3 => "40".to_string(),
            _ => "".to_string(),
        };
        write!(f, "{}", state_str)
    }
}

#[derive(Clone, Copy)]
struct Match {
    p1: Player,
    p2: Player,
}

impl Match {
    // Constructor para Player
    fn new(p1: Player, p2: Player) -> Match {
        Match { p1, p2 }
    }
}

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_player_point = std::cmp::max(self.p1.point, self.p2.point);
        let difference_between_player = (self.p1.point - self.p2.point).abs();
        if max_player_point > 3 {
            if self.p1.point == self.p2.point {
                write!(f, "Deuce")
            } else if difference_between_player < 2 {
                write!(
                    f,
                    "Advantage {}",
                    if self.p1.point > self.p2.point {
                        self.p1.name
                    } else {
                        self.p2.name
                    }
                )
            } else {
                write!(
                    f,
                    "Win {}",
                    if self.p1.point > self.p2.point {
                        self.p1.name
                    } else {
                        self.p2.name
                    }
                )
            }
        }
         else {
            if self.p1.point == self.p2.point && self.p1.point == 3 {
                write!(f, "Deuce")
            } else {
                write!(f, "{} - {}", self.p1, self.p2)
            }
        }
    }
}


fn main() {
    let match_track = [
        PlayerName::P1,
        PlayerName::P1,
        PlayerName::P2,
        PlayerName::P2,
        PlayerName::P1,
        PlayerName::P2,
        PlayerName::P1,
        PlayerName::P1,
    ];

    let  players_match = Match::new(
        Player::new(PlayerName::P1, 0),
        Player::new(PlayerName::P2, 0),
    );
    match_tracking(&players_match,&match_track);

   
   
}

fn match_tracking(inital_match: &Match,match_track:&[PlayerName]){
    let mut player_match = inital_match.clone();

    for player in match_track {
        player_match = match player {
            PlayerName::P1 => Match {
                p1: player_match.p1.add_point_to_player(),
                p2: player_match.p2,
            },
            PlayerName::P2 => Match {
                p1: player_match.p1,
                p2: player_match.p2.add_point_to_player(),
            },
        };
        println!("{}", player_match);
    }
}

trait AddPoint {
    fn add_point_to_player(&self) -> Player;
}

impl AddPoint for Player {
    fn add_point_to_player(&self) -> Player {
        Player {
            name: self.name.clone(),
            point: self.point + 1,
        }
    }
}
