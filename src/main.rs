mod format;
use format::*;
mod configure;
use configure::*;

fn main() {
    let args = Args::parse();
    let _deck = Deck::new(file_to_cards(&args.deck),
    args.deck_type);
}

