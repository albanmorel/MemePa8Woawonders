use crate::card::Card;
use crate::card::RessourceType;
use crate::card::Cost;
use crate::card::CardType;
use rand::seq::SliceRandom;
use rand::thread_rng;


#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Wonder {
    name: String,
    stages: [Card; 3],
    ressource: RessourceType,
}
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Wonders(Vec<Wonder>);
impl Wonders {
    pub fn shuffle_wonders(&mut self) {
        self.0.shuffle(&mut thread_rng());
    }

    pub fn get_random_wonder(&mut self) -> Wonder {
        self.0.pop().unwrap()
    }
}

pub fn get_wonders() -> Wonders {
    let mut wonders_list: Vec<Wonder> = Vec::new();
    wonders_list.push(Wonder {
        name: "The Pyramids of Giza".to_string(),
        stages: [
            Card::new(
                "stage 1".to_string(), 
                Cost(vec![RessourceType::Stone, RessourceType::Stone]), 
                CardType::Blue { victory_point: 3 }
            ),
            Card::new(
                "stage 2".to_string(), 
                Cost(vec![RessourceType::Wood,RessourceType::Wood,RessourceType::Wood]), 
                CardType::Blue { victory_point: 5 }
            ),
            Card::new(
                "stage 3".to_string(), 
                Cost(vec![RessourceType::Stone, RessourceType::Stone,RessourceType::Stone,RessourceType::Stone]), 
                CardType::Blue { victory_point: 7 }
            ),
        ],
        ressource: RessourceType::Stone,
    });
    wonders_list.push(Wonder {
        name: "The Colossus of Rhodes".to_string(),
        stages: [
            Card::new(
                "stage 1".to_string(), 
                Cost(vec![RessourceType::Wood, RessourceType::Wood]), 
                CardType::Blue { victory_point: 3 }
            ),
            Card::new(
                "stage 2".to_string(), 
                Cost(vec![RessourceType::Clay,RessourceType::Clay,RessourceType::Clay]), 
                CardType::Red { combat_point: 2}
            ),
            Card::new(
                "stage 3".to_string(), 
                Cost(vec![RessourceType::Ore, RessourceType::Ore,RessourceType::Ore,RessourceType::Ore]), 
                CardType::Blue { victory_point: 7 }
            ),
        ],
        ressource: RessourceType::Ore,
    });
    wonders_list.push(Wonder {
        name: "The Temple of Artemis in Ephesus".to_string(),
        stages: [
            Card::new(
                "stage 1".to_string(), 
                Cost(vec![RessourceType::Stone, RessourceType::Stone]), 
                CardType::Blue { victory_point: 3 }
            ),
            Card::new(
                "stage 2".to_string(), 
                Cost(vec![RessourceType::Wood,RessourceType::Wood]), 
                CardType::Yellow { gold_gain: 9}
            ),
            Card::new(
                "stage 3".to_string(), 
                Cost(vec![RessourceType::Papyrus, RessourceType::Papyrus]), 
                CardType::Blue { victory_point: 7 }
            ),
        ],
        ressource: RessourceType::Papyrus,
    });
    Wonders(wonders_list)
}
