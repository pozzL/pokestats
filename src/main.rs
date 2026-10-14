use std::env;
use crate::formatting::Pokemon;

mod formatting;
mod like;

const DATA: &str = include_str!("../data/poke_stats.csv");


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: pokestats <pokemon>");
        std::process::exit(1);
    }

    let mut reader = csv::Reader::from_reader(DATA.as_bytes());
    let mut pokemons: Vec<Pokemon> = Vec::new();

    for result in reader.deserialize() {
        pokemons.push(result?);
    }

    match like::closest(&args[1], &pokemons) {
        Some(pokemon) => println!("{}", pokemon),
        None => eprintln!("no Pokémon found"),
    }

    Ok(())
}
