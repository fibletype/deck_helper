use clap::Parser;
use std::{vec};
use std::fs::File;
use std::io::prelude::*;

/// Simple program to greet a person


enum DeckType {
    Aggro,
    Midrange,
    Control,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the file with deck list
    #[arg(short, long)]
    deck: String,

    #[arg(short, long)]
    turn: u8,

    #[arg(short, long)]
    deck_type: String,
}

struct Card {
    /// Card name
    name: String,
    ///need to rework to struct
    cmc : u8, 
}

struct Deck {
    ///
    cards: Vec<Card>,
    deck_type: DeckType,
}

fn str_to_deck_type (str : String) -> DeckType {
    unimplemented!("str_to_deck_type")
}

fn file_to_cards(f: String) -> Vec<Card> {
    let deck_list = File::open(f);
    let mut contents = String::new();
    if let Ok(mut f) = deck_list {
        match f.read_to_string(&mut contents) {
            Ok(_b) => unimplemented!("Need to create deck"),
            _             => panic!("some strange error =)"),
            }
    } else {
        panic!("deck list doesn't exists in file system")
    }
}

impl Deck {
    pub const fn new(card_list : Vec<Card>, deck_type: DeckType) -> Self {
        Deck {cards : card_list, deck_type}
    }
}

fn main() {
    let args = Args::parse();
    let deck = Deck::new(file_to_cards(args.deck),
                               str_to_deck_type(args.deck_type));
}