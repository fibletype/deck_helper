
use std::fmt::format;

use crate::format::*;

fn colors_to_str (colors : Vec<String>) -> String {
  let mut buf = "".to_owned();
  for i in colors {
    match i.as_str() {
      "B" => buf += "Black",
      "U" => buf += "Blue",
      "W" => buf += "White",
      "R" => buf += "Red",
      "G" => buf += "Green",
      _ => panic!("Wrong deck colors"),
    }
  }
  buf
}

fn one_color_deck (deck : &Deck) {
  let n = deck.number_of_cards();
  match deck.deck_type {
    DeckType::Commander => println!("You need {} {} lands", 100 - n, colors_to_str(deck.deck_colors())),
    DeckType::Modern    => println!("You need {} {} lands", 60  - n, colors_to_str(deck.deck_colors())),
    DeckType::Draft     => println!("You need {} {} lands", 40  - n, colors_to_str(deck.deck_colors())),
  }
}

fn two_color_deck (deck : &Deck) {
  unimplemented!("two color deck");
}

pub fn adding_lands (deck : &Deck) {
  match deck.deck_colors().len() {
    1 => one_color_deck(deck),
    2 => two_color_deck(deck),
    _ => unimplemented!("to much colors in your deck"),
  }
} 

pub fn analyzing_lands (deck : Deck) {
  unimplemented!("analize lands option unavaliable");
}

/// Return probability of n lands in start hand
pub fn lands_probability (deck : Deck, l : u8) -> f32 {
  if l > 7 {
    panic!("You realy need more than 7 lands in your start hand?")
  }
  let n = deck.number_of_cards();
  let lands = deck.number_of_card_type("land");
  let other = n - lands;
  /// TODO rewrite if posbille
  let mut num = 1;
  for i in 0..l {
    num *= (lands - i) / (i + 1)
  }
  for i in 0..7-l {
    num *= (other - i) / (i + 1)
  }
  let mut den = 1;
  for i in 0..7 {
    den *= (n - i) / (i + 1)
  }
  (100.0 * num as f32 / den as f32)
}

/// Return probability of 2 or 3 lands in start hand both colors
pub fn two_color_probability (deck : &Deck) -> f32 {
  let n = deck.number_of_cards();
  let colors = deck.deck_colors();
  if colors.len() != 2 {
    panic!("Need exact 2 colors");
  }
  let lands_first = deck.number_of_card_oracle(format!("{{T}}: Add {{{color}}}.", color = colors[0]));
  let lands_second = deck.number_of_card_oracle(format!("{{T}}: Add {{{color}}}.", color = colors[0]));
  let lands_dual = deck.number_of_card_oracle(format!("{{T}}: Add {{{color1}}} or {{{color2}}}.", color1 = colors[0], color2 = colors[1])) +
                   deck.number_of_card_oracle(format!("{{T}}: Add {{{color2}}} or {{{color1}}}.", color1 = colors[0], color2 = colors[1]));
  // TODO handle more specific lands
  let num_2 = (lands_first * lands_second +
                  lands_second * lands_dual +
                  lands_first * lands_dual +
                  lands_dual * (lands_dual - 1)) / 2;
  let num_3 = (lands_first * lands_second * lands_dual +
                  lands_second * lands_dual * (lands_dual - 1) +
                  lands_first * lands_dual * (lands_dual - 1) +
                  lands_dual * (lands_dual - 1) * (lands_dual - 2) +
                  lands_first * lands_second * (lands_second - 1) +
                  lands_first * lands_second * (lands_first - 1) +
                  lands_first * lands_dual * (lands_first - 1) +
                  lands_dual * lands_second * (lands_second - 1)) / 6;
  let mut den = 1;
  for i in 0..7 {
    den *= (n - i) / (i + 1)
  }
  100.0 * (num_2 + num_3) as f32 / den as f32
}