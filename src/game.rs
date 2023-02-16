use std::collections::HashMap;

use colored::Colorize;
use crate::card::RessourceType;
use crate::player::Player;
use crate::card::Card;

pub fn display_cards(player: &Player){
    for i in 0..player.get_hand().len(){
        print!("carte n°{} ", i);
        player.get_hand()[i].display();        
    }
}

pub fn play_card(card: Card, player: &mut Player){
    player.add_card_to_board(card);
    
}

pub fn check_resourses_card(cost: Vec<RessourceType>, player: Player){
    let map = cost.iter().fold(HashMap::<RessourceType, u32>::new(), |mut acc, &x| {
        *acc.entry(x).or_default() += 1;
        acc
    });
}

pub fn choose_card(card: Card, player: Player){

}