//! The exercise routine the physical therapist prescribed, as data.
//!
//! The app never hard-codes the exercises: it asks for [`default_routine`] and could later load
//! a routine the user edited (longer holds, more reps).

use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// A full session: the exercises, done in order.
#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Routine {
    pub exercises: Vec<Exercise>,
}

/// One exercise: a movement repeated `reps` times, with a rest between repetitions.
#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Exercise {
    pub movement: Movement,
    pub reps: u32,
    /// Pause between repetitions (not after the last one).
    pub rest_ms: u32,
}

/// What one repetition of an exercise consists of.
#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum Movement {
    /// Squeeze up gradually, then let go gradually.
    SlowSqueeze { squeeze_ms: u32, release_ms: u32 },
    /// Hold a squeeze, then let go.
    Hold { hold_ms: u32, release_ms: u32 },
    /// A run of short squeezes.
    QuickSqueezes {
        count: u32,
        squeeze_ms: u32,
        release_ms: u32,
    },
}

impl Movement {
    /// How many squeezes one repetition contains.
    pub fn squeezes(&self) -> u32 {
        match self {
            Movement::SlowSqueeze { .. } | Movement::Hold { .. } => 1,
            Movement::QuickSqueezes { count, .. } => *count,
        }
    }
}

/// The prescribed routine: slow squeezes, long holds, quick squeezes; ten of each.
pub fn default_routine() -> Routine {
    Routine {
        exercises: vec![
            Exercise {
                movement: Movement::SlowSqueeze {
                    squeeze_ms: 5_000,
                    release_ms: 5_000,
                },
                reps: 10,
                rest_ms: 3_000,
            },
            Exercise {
                movement: Movement::Hold {
                    hold_ms: 10_000,
                    release_ms: 3_000,
                },
                reps: 10,
                rest_ms: 0,
            },
            Exercise {
                movement: Movement::QuickSqueezes {
                    count: 10,
                    squeeze_ms: 500,
                    release_ms: 500,
                },
                reps: 10,
                rest_ms: 3_000,
            },
        ],
    }
}

/// Pretzelometer rotomies earned by one full session: one per squeeze.
pub fn rotomies(routine: &Routine) -> u32 {
    routine
        .exercises
        .iter()
        .map(|e| e.reps * e.movement.squeezes())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_prescribed_routine_has_three_exercises_of_ten_reps() {
        let routine = default_routine();
        assert_eq!(routine.exercises.len(), 3);
        assert!(routine.exercises.iter().all(|e| e.reps == 10));
    }

    #[test]
    fn every_squeeze_is_worth_one_rotomy() {
        // 10 slow + 10 holds + 10 runs of 10 quick squeezes
        assert_eq!(rotomies(&default_routine()), 120);
    }
}
