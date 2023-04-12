
mod format;
use crate::format::*;

fn colors_to_str (colors : Vec<String>) -> String {
  unimplemented!();
}

fn one_color_deck (deck : Deck) {
  match deck.deck_type {
    Commander => println!("You need {} {} lands", 100 - deck.number_of_cards(), colors_to_str(deck.deck_colors())),
    Modern    => println!("You need {} {} lands", 60  - deck.number_of_cards(), colors_to_str(deck.deck_colors())),
    Draft     => println!("You need {} {} lands", 40  - deck.number_of_cards(), colors_to_str(deck.deck_colors())),
  }
}

fn two_color_deck (deck : Deck) {
  unimplemented!("two color deck");
}

pub fn adding_lands (deck : Deck) {
  match deck.deck_colors().len() {
    1 => one_color_deck(deck),
    2 => two_color_deck(deck),
    _ => unimplemented!("to much colors in your deck"),
  }
} 

pub fn analyzing_lands (deck : Deck) {
  unimplemented!("analize lands option unavaliable ");
}