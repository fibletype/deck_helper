use std::path::Path;
use std::str::FromStr;
use std::fs::File;
use std::io::prelude::*;

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

#[derive(Debug, PartialEq)]
pub struct Card {
    /// numbers of card
    num: u8,
    /// Card name
    name: String,
    ///need to rework to pub struct
    cmc : u8, 
}


pub struct Deck {
  ///
  cards: Vec<Card>,
  deck_type: DeckType,
}

impl Deck {
  pub const fn new(card_list : Vec<Card>, deck_type: DeckType) -> Self {
      Deck {cards : card_list, deck_type}
  }
}
/* TODO how to use resulting */
pub fn file_to_cards(f: &Path) -> Vec<Card> {
  let deck_list = File::open(f);
  let mut contents = String::new();
  if let Ok(mut f) = deck_list {
      match f.read_to_string(&mut contents) {
          Ok(_b) => str_to_deck(contents),
          _             => panic!("some strange error =)"),
          }
  } else {
      panic!("deck list doesn't exists in file system")
  }
}

fn str_to_deck (str : String) -> Vec<Card> {
  let mut deck = Vec::new();
  for i in str.lines().into_iter() {
      let items : Vec <_>  = i.split_whitespace().collect();
      let num : u8 = items[0].parse().unwrap();
      let cmc : u8 = items.last().unwrap().parse().unwrap();
      let name: String = items[1..(items.len() - 1)].join(" ");
      deck.push(Card {num:  num,
                      name: name,
                      cmc:  cmc,
               })
  }
  deck
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

    #[test]
    fn test_to_deck_0() {
        assert_eq!(str_to_deck("4 Card_name 3".to_string()), [Card {num: 4 ,
                                                                   name: "Card_name".to_string(),
                                                                   cmc:  3,}]);
    }

    #[test]
    fn test_to_deck_1() {
        assert_eq!(str_to_deck("4 Card name 3".to_string()), [Card {num: 4 ,
                                                                   name: "Card name".to_string(),
                                                                   cmc:  3,}]);
    }
}
