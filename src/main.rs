use cmudict_fast as cmudict;
use cmudict::Cmudict;
use cmudict_fast::{rhymes, Rule};

fn main() {
    println!("Hello, world!");
    rhymer();
}

fn rhymer() -> Option<Rule> {
    let dict = Cmudict::new("./resources/cmudict.dict").ok()?;
    let acted = dict.get("ACTED");
    let reacted = dict.get("REACTED");
    let acrimonius = dict.get("ACRIMONIOUS");

    println!("{}", acted.is_some());
    let answer = rhymes(acted.unwrap(), reacted.unwrap());
    println!("{}", answer);
    let answer2 = rhymes(acted.unwrap(), acrimonius.unwrap());
    println!("{}", answer2);
    return Option::None;
}