use crate::{gamedata::registry::Registry, battles::battle::Battle, cardparse::enums::Outcome};
use std::{fmt::Display, cmp::Ordering};

/// Wraps a Vec<&str> deck, keeps track of deck ELO
#[derive(Debug)]
pub struct Elo<'a> {
    pub deck: Vec<&'a str>,
    pub elo: f32,
}

impl<'a> Elo<'a> {
    pub fn new(deck: Vec<&'a str>) -> Self {
        Self {
            deck,
            elo: 1000.0,
        }
    }

    pub fn battle(&mut self, other: &mut Self, reg: &Registry) {
        let mut battle = Battle::new(reg, &self.deck, &other.deck);
        let outcome = battle.game();
        self.outcome_of_battle(other, outcome);
    }

    pub fn prob_win(&self, other: &Self) -> f32 {
        let step1 = other.elo as f32 - self.elo as f32;
        let step2 = step1 / 400.0;
        let step3 = f32::powf(10.0, step2);
        1.0 / (1.0 + step3)
    }

    pub fn new_elo(&mut self, outcome_score: f32, prob_win: f32) {
        let step1: f32 = outcome_score - prob_win;
        let step2: f32 = 32.0 * step1;
        self.elo += step2;
    }

    pub fn outcome_of_battle(&mut self, other: &mut Self, outcome: Outcome) {
        let outcome_score = outcome.as_f32();
        let prob_win = self.prob_win(other);

        self.new_elo(outcome_score, prob_win);
        other.new_elo(1.0 - outcome_score, 1.0 - prob_win);
    }
}

impl<'a> Display for Elo<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) {:?}", self.elo, self.deck)
    }
}

impl<'a> Ord for Elo<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.elo.total_cmp(&self.elo)
    }
}

impl<'a> PartialOrd for Elo<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Elo<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.elo == other.elo
    }
}

impl<'a> Eq for Elo<'a> {}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn elo_test() {
        let mut home = Elo::new(Vec::new());
        let mut other = Elo::new(Vec::new());
        home.elo = 1100.0;
        other.elo = 900.0;
        assert_eq!(home.prob_win(&other), 0.7597469);
    }

    #[test]
    fn elo_battle_test() {
        let mut home = Elo::new(Vec::new());
        let mut other = Elo::new(Vec::new());
        home.elo = 1656.0;
        other.elo = 1763.0;
        home.outcome_of_battle(&mut other, Outcome::Win);
        assert_eq!(home.elo, 1676.7775);
        assert_eq!(other.elo, 1742.2225);
    }
}
