use std::fmt::Display;

use crate::types::AppResult;

use super::action::Action;
use rand::seq::IteratorRandom;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, WeightedIndex};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, Default, Serialize_repr, Deserialize_repr, PartialEq, EnumIter)]
#[repr(u8)]
pub enum Tactic {
    #[default]
    Balanced,
    BigPirates,
    Arrembaggio,
}

impl Display for Tactic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tactic::Balanced => write!(f, "Balanced"),
            Tactic::BigPirates => write!(f, "Big Pirates"),
            Tactic::Arrembaggio => write!(f, "Arrembaggio"),
        }
    }
}

impl Tactic {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self::iter().choose(&mut rng).unwrap()
    }

    pub fn next(&self) -> Self {
        match self {
            Self::Balanced => Self::BigPirates,
            Self::BigPirates => Self::Arrembaggio,
            Self::Arrembaggio => Self::Balanced,
        }
    }

    pub fn pick_action(&self, rng: &mut ChaCha8Rng) -> AppResult<Action> {
        let weights = match self {
            Self::Balanced => [2, 2, 3, 2],
            Self::BigPirates => [2, 1, 1, 3],
            Self::Arrembaggio => [2, 3, 3, 1],
        };
        let idx = WeightedIndex::new(&weights)?.sample(rng);
        let action = match idx {
            0 => Action::Isolation,
            1 => Action::OffTheScreen,
            2 => Action::PickAndRoll,
            3 => Action::Post,
            _ => return Err("Invalid index in pick_action.".into()),
        };
        Ok(action)
    }

    pub fn brawl_probability(&self) -> f32 {
        match self {
            Self::Balanced => 0.1,
            Self::BigPirates => 0.2,
            Self::Arrembaggio => 0.3,
        }
    }
}
