/// Custom iyes pickers
use bevy::prelude::*;
use big_brain::{choices::Choice, prelude::Picker, scorers::Score};

// NOTE nm, found that theres a .otherwise method for thinkers
/*
/// Takes only two choices, picks first if past threshold, otherwise picks second
#[derive(Debug, Clone, Default)]
pub struct ScoreOr {
    pub threshold: f32
}

impl ScoreOr {
    pub fn new(threshold: f32) -> Self {
        Self { threshold }
    }
}

impl Picker for ScoreOr {
    fn pick<'a>(&self, choices: &'a [Choice], scores: &Query<&Score>) -> Option<&'a Choice> {
        if choices.len

        for choice in choices {
            let value = choice.calculate(scores);
            if value >= self.threshold {
                return Some(choice);
            }
        }
        None
    }
}
*/
