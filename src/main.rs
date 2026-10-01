use std::fs::File;
use std::env;
use crate::formatting::Pokemon;

mod formatting;

const PATH: &str = "data/poke_stats.csv";


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    let file = File::open(PATH)?;

    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
        let pokemon: Pokemon = result?;
        if pokemon.name==args[1] {
            println!("{}",pokemon);
        }
    }
    Ok(())
}
