mod format;
use format::*;
mod configure;
use configure::*;
mod logic;
use logic::lands;


fn main() {
    // let args = Args::parse();
    // let deck = Deck::new(file_to_cards(&args.deck).unwrap(), args.deck_type);
    let response = reqwest::blocking::get( "https://api.scryfall.com/cards/named?exact=Raugrin Triome").unwrap();
    let resp: SkyfallCard = response.json().unwrap();
    println!("{:?}", resp);
      
}