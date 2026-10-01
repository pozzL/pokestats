use serde::Deserialize;
use std::fmt;

#[derive(Debug,Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Pokemon {
    pub number: i16,
    pub typeone: String,
    pub typetwo: String,
    pub total: i16,
    pub hp: u8,
    pub generation: u8,
    pub legendary: u8,
    pub name: String,
    pub attack: u8,
    pub defense: u8,
    pub spattack: u8,
    pub spdefense: u8,
    pub speed: u8,
}


impl fmt::Display for Pokemon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bar_single = |val: u8| -> String {
            "=".repeat((val.max(0) / 10) as usize)
        };

        let bar_total = |val: i16| -> String {
            "=".repeat((val.max(0) / 60) as usize)
        };

        writeln!(f, "— {} N.{} —", self.name,self.number)?;
        writeln!(f, "Generazione: {}", self.generation)?;
        
        if self.typetwo != "" {
            writeln!(f, "Tipo:        {} / {}", self.typeone, self.typetwo)?;
        } else {
            writeln!(f, "Tipo:        {}", self.typeone)?;
        }
        
        writeln!(f, "Leggendario: {}", if self.legendary>0 { "Sì" } else { "No" })?;
        writeln!(f, "———————————————————————————————")?;

        // Tutti i campi sono ora in minuscolo per corrispondere alla struct
        writeln!(f, "HP:       {:3} \"{}\"", self.hp, bar_single(self.hp))?;
        writeln!(f, "Attacco:  {:3} \"{}\"", self.attack, bar_single(self.attack))?;
        writeln!(f, "Difesa:   {:3} \"{}\"", self.defense, bar_single(self.defense))?;
        writeln!(f, "Sp. Atk:  {:3} \"{}\"", self.spattack, bar_single(self.spattack))?;
        writeln!(f, "Sp. Def:  {:3} \"{}\"", self.spdefense, bar_single(self.spdefense))?;
        writeln!(f, "Velocità: {:3} \"{}\"", self.speed, bar_single(self.speed))?;
        
        writeln!(f, "———————————————————————————————")?;
        writeln!(f, "TOTALE:   {:3} \"{}\"", self.total, bar_total(self.total))
    }
}



