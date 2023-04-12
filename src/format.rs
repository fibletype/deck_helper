use core::time;
use std::path::Path;
use std::str::FromStr;
use std::fs::File;
use std::io::prelude::*;
use serde::Deserialize;

#[derive(Debug, PartialEq, Clone)]
pub enum DeckType {
    Aggro,
    Midrange,
    Control,
}

impl FromStr for DeckType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "aggro" => Ok(DeckType::Aggro),
            "midrange" => Ok(DeckType::Midrange),
            "control" => Ok(DeckType::Control),
            _ => Err("Wrong deck type"),
        }
    }
}


#[derive (Debug, Deserialize, PartialEq)]
pub struct SkyfallCard {
    name : String,
    cmc : f32,
    mana_cost : String,
    type_line : String,
    oracle_text : String, 
}
#[derive(Debug, PartialEq)]
pub struct Card {
    /// numbers of card
    num: u8,
    /// Card name
    card : SkyfallCard,
}


pub struct Deck {
  ///
  cards: Vec<Card>,
  deck_type: DeckType,
}

impl Deck {
  pub const fn new(cards : Vec<Card>, deck_type: DeckType) -> Self {
      Deck {cards, deck_type}
  }
}

pub fn file_to_cards(f: &Path) -> Result<Vec<Card>> {
  let deck_list = File::open(f);
  let mut contents = String::new();
  if let Ok(mut f) = deck_list {
    f.read_to_string(&mut contents).map_err(|err| format!("ERR: {err}"))?;
    str_to_deck(contents)
  } else {
      Err("deck list doesn't exists in file system".into())
  }
}

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn str_to_deck (str : String) -> Result<Vec<Card>> {
  let mut deck = Vec::new();
  for i in str.lines().into_iter() {
      let items : Vec <_>  = i.split_whitespace().collect();
      let num : u8 = items[0].parse().unwrap();
      let name = items[1..items.len()].join(" ");
      let response = reqwest::blocking::get( "https://api.scryfall.com/cards/named?exact=".to_owned() + &name).unwrap();
      if response.status() == 200 {
        let resp: SkyfallCard = response.json().unwrap();
        deck.push(Card {num, card : resp});
      } else { 
        println!("Card \"{}\" doesn't exists, please amend name\n", name)
      }
      std::thread::sleep(time::Duration::from_millis(100));
  }
  Ok(deck)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_deck_type() {
        assert_eq!(DeckType::from_str("AgGro"), Ok(DeckType::Aggro));
        assert_eq!(DeckType::from_str("MiDraNge"), Ok(DeckType::Midrange));
        assert_eq!(DeckType::from_str("ContRol"), Ok(DeckType::Control));
    }

    #[test]
    fn parse_deck_type_wrong() {
        assert!(DeckType::from_str("wrong").is_err());
    }
}
