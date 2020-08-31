use eliza::Eliza;
use std::error::Error;

pub struct ElizaBot {
    eliza: Eliza,
}

impl ElizaBot {
    pub fn new() -> Result<ElizaBot, Box<dyn Error>> {
        let script = include_str!("../scripts/doctor.json");
        let eliza = Eliza::from_str(script)?;
        Ok(ElizaBot { eliza })
    }

    pub fn respond(&mut self, input: &str) -> String {
        self.eliza.respond(input)
    }
}
