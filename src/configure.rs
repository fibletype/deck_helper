pub use clap::Parser;
use std::path::{PathBuf};

use crate::format::DeckType;



#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the file with deck list
    #[arg(long)]
    pub deck: PathBuf,
    /// number of turn
    #[arg(short, long)]
    pub turn: u8,
    ///Your deck type
    #[arg(short, long)]
    pub deck_type: DeckType,
}



