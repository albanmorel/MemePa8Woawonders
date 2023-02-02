use colored::Colorize;
use std::vec::Vec;
use rand::thread_rng;
use rand::Rng;
use rand::seq::SliceRandom;


struct Player {
    id: u32,
    hand: Vec<Card>,
    board: Vec<Card>,
    wonder: Wonder,
}

impl Player{
    pub fn new_player(id: u32, wonders: &mut Wonders) -> Player {
        Player { id: id, hand: Vec::new(), board: Vec::new(), wonder: wonders.get_random_wonder() }
    }
}
struct Card {
    name: String,
    cost : Cost,
    card_type: CardType,    
}

enum CardType {
    Blue {
        victory_point: u32,
    },
    Green{
        Symbol: GreenType,
    },
    Yellow {
        gold_gain: u32,
    },
    Gray {
        s_ressource_type: RessourceType,
    },
    Brown {
        ressource_type: RessourceType,
    },
    Red {
        combat_point: u32,
    },
}

enum GreenType {
    Wheel,
    Tablet,
    Ruler,
}

enum RessourceType {
    Stone,
    Wood,
    Minerals,
    Clay,
}
struct Cost(Vec<RessourceType>);
struct Wonder {
    name: String,
    stages: [CardType; 3],
    ressource: RessourceType,
}

struct Wonders(Vec<Wonder>);
impl Wonders{
    pub fn shuffle_wonders(&mut self) {
        self.0.shuffle(&mut thread_rng());
    }

    pub fn get_random_wonder(&mut self) -> Wonder {
        self.0.pop().unwrap()      
    }
}

fn get_nb_of_player() -> u32 {
    loop {
        println!("Combien de joueur meirveilleux qui jouent ?");
        let mut user_input: String = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();

        match user_input.trim().parse::<u32>() {
            Ok(user_input) if user_input <= 7 => return user_input,
            Ok(_user_input) => {
                println!(
                    "{}",
                    ("Yoo very desoled mais c'est 7 joueurs maximum :/ (C'est dans le titre du jeu, fais un effort)")
                        .italic()
                        .red()
                );
                continue;
            }
            Err(_e) => {
                println!("{}", ("Yooo g pa kompri").italic().red());
                continue;
            }
        };
    }
}

fn create_players(nb_of_player: u32) -> Vec<Player>{
    let player_list: Vec<Player> = Vec::new();
    player_list
}
fn main() {
    println!("{:?}", get_nb_of_player());
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