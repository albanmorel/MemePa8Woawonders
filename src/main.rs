use colored::Colorize;
use std::vec::Vec;
mod wonders;
mod card;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Player {
    id: u32,
    hand: Vec<card::Card>,
    board: Vec<card::Card>,
    wonder: wonders::Wonder,
}

impl Player {
    pub fn new_player(id: u32, wonder: wonders::Wonder) -> Player {
        Player {
            id,
            hand: Vec::new(),
            board: Vec::new(),
            wonder,
        }
    }
}

fn get_nb_of_player() -> u32 {
    loop {
        println!("Combien de joueur meirveilleux qui jouent ?");
        let mut user_input: String = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();

        match user_input.trim().parse::<u32>() {
            Ok(user_input) if user_input <= 7 => return user_input,
            Ok(_) => {
                println!(
                    "{}",
                    ("Yoo very desoled mais c'est 7 joueurs maximum :/ (C'est dans le titre du jeu, fais un effort)")
                        .italic()
                        .red()
                );
                continue;
            }
            Err(_) => {
                println!("{}", ("Yooo g pa kompri").italic().red());
                continue;
            }
        };
    }
}

fn create_players(nb_of_player: u32, wonders: &mut wonders::Wonders) -> Vec<Player> {
    let mut player_list: Vec<Player> = Vec::new();
    for i in 0..nb_of_player {
        player_list.push(Player::new_player(i + 1, wonders.get_random_wonder()))
    }
    player_list
}
fn main() {
    let mut wonders = wonders::get_wonders();
    let players_list = create_players(get_nb_of_player(), &mut wonders);

    for player in players_list {
        println!("{:?}", player)
    }
    //println!("{:?}", get_nb_of_player());
}
//Savoir combien de joueurs jouent
//Creer un deck approprié au nombre de joueur
//Attribuer une merveille à chaque joueurs
//Distribuer les cartes aux joueurs
//Jouer un tour
//Chaque joueurs selectionne une carte
//Vérifier s'il peut la jouer
//Jouer simultanément toutes les cartes et resoudres leurs effets
//Passer la main de chaque joueur à son voisin de droite ou gauche
//Quand plus qu'une carte par main
//defausse
//Résoudre les combats
//distribuer carte de l'âge d'après
//Quand toutes les cartes de tous les âges ont été jouées, compter les points et nommer le gagnant
