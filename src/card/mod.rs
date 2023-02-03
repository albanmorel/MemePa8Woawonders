#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Card {
    name: String,
    cost: Cost,
    card_type: CardType,
}

impl Card{
    pub fn new(name: String, cost: Cost, card_type: CardType) -> Self {
        Self {name, cost, card_type}
    }
}
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum CardType {
    Blue { victory_point: u32 },
    Green { Symbol: GreenType },
    Yellow { gold_gain: u32 },
    Gray { ressource_type: RessourceType },
    Brown { ressource_type: RessourceType },
    Red { combat_point: u32 },
}
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
enum GreenType {
    Wheel,
    Tablet,
    Ruler,
}
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum RessourceType {
    Stone,
    Wood,
    Ore,
    Clay,
    Glass,
    Papyrus,
    Textile,
}
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Cost(pub Vec<RessourceType>);