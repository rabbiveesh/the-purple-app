//! Turns a routine into a timed session: what happens when, and what the coach says.
//!
//! The player in the app only has to look up the step for the elapsed time and play the cues
//! as their moment passes; every timing rule lives here.

use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::routine::{Movement, Routine};

/// Timing choices that are not part of the prescribed routine itself.
#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlanOptions {
    /// The "exercise N starts in 5 seconds" lead-in before each exercise.
    pub get_ready_ms: u32,
    /// The short "done" breather between exercises.
    pub exercise_done_ms: u32,
}

impl Default for PlanOptions {
    fn default() -> Self {
        PlanOptions {
            get_ready_ms: 5_000,
            exercise_done_ms: 2_000,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Phase {
    GetReady,
    Squeeze,
    Release,
    Rest,
    ExerciseDone,
}

/// One stretch of the session with a single instruction on screen.
#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    /// 1-based exercise number.
    pub exercise: u32,
    /// 1-based repetition, or 0 for the lead-in and the "done" breather.
    pub rep: u32,
    pub phase: Phase,
    pub start_ms: u32,
    pub duration_ms: u32,
}

/// Something the audio coach says.
#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Say {
    GetReady { exercise: u32 },
    Squeeze,
    Release,
    Rest,
    Count { n: u32 },
    ExerciseDone { exercise: u32 },
    SessionDone,
}

#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Cue {
    pub at_ms: u32,
    pub say: Say,
}

#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SessionPlan {
    /// Back to back, in order, covering 0..total_ms.
    pub steps: Vec<Step>,
    /// In time order.
    pub cues: Vec<Cue>,
    pub total_ms: u32,
    pub exercise_count: u32,
}

/// Lays the routine out on a timeline.
pub fn session_plan(routine: &Routine, options: &PlanOptions) -> SessionPlan {
    let mut b = Builder::default();
    let count = routine.exercises.len() as u32;

    for (i, exercise) in routine.exercises.iter().enumerate() {
        let n = i as u32 + 1;
        b.say(Say::GetReady { exercise: n });
        b.step(n, 0, Phase::GetReady, options.get_ready_ms);

        for rep in 1..=exercise.reps {
            match exercise.movement {
                Movement::SlowSqueeze {
                    squeeze_ms,
                    release_ms,
                } => {
                    b.counted(n, rep, Phase::Squeeze, Say::Squeeze, squeeze_ms);
                    b.counted(n, rep, Phase::Release, Say::Release, release_ms);
                }
                Movement::Hold {
                    hold_ms,
                    release_ms,
                } => {
                    b.counted(n, rep, Phase::Squeeze, Say::Squeeze, hold_ms);
                    b.counted(n, rep, Phase::Release, Say::Release, release_ms);
                }
                Movement::QuickSqueezes {
                    count,
                    squeeze_ms,
                    release_ms,
                } => {
                    for k in 1..=count {
                        b.say(Say::Count { n: k });
                        b.step(n, rep, Phase::Squeeze, squeeze_ms);
                        b.step(n, rep, Phase::Release, release_ms);
                    }
                }
            }
            if rep < exercise.reps && exercise.rest_ms > 0 {
                b.say(Say::Rest);
                b.step(n, rep, Phase::Rest, exercise.rest_ms);
            }
        }

        if n < count {
            b.say(Say::ExerciseDone { exercise: n });
            b.step(n, 0, Phase::ExerciseDone, options.exercise_done_ms);
        }
    }
    b.say(Say::SessionDone);

    SessionPlan {
        steps: b.steps,
        cues: b.cues,
        total_ms: b.now,
        exercise_count: count,
    }
}

#[derive(Default)]
struct Builder {
    steps: Vec<Step>,
    cues: Vec<Cue>,
    now: u32,
}

impl Builder {
    fn say(&mut self, say: Say) {
        self.cues.push(Cue {
            at_ms: self.now,
            say,
        });
    }

    fn step(&mut self, exercise: u32, rep: u32, phase: Phase, duration_ms: u32) {
        if duration_ms == 0 {
            return;
        }
        self.steps.push(Step {
            exercise,
            rep,
            phase,
            start_ms: self.now,
            duration_ms,
        });
        self.now += duration_ms;
    }

    /// A phase announced by name and then counted out loud each second: "squeeze, 2, 3, 4, 5".
    fn counted(&mut self, exercise: u32, rep: u32, phase: Phase, word: Say, duration_ms: u32) {
        let start = self.now;
        self.say(word);
        for second in 1..duration_ms.div_ceil(1_000) {
            self.cues.push(Cue {
                at_ms: start + second * 1_000,
                say: Say::Count { n: second + 1 },
            });
        }
        self.step(exercise, rep, phase, duration_ms);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routine::default_routine;

    fn plan() -> SessionPlan {
        session_plan(&default_routine(), &PlanOptions::default())
    }

    #[test]
    fn a_session_takes_under_seven_minutes() {
        // 3 lead-ins of 5 s and 2 breathers of 2 s, then 10 reps of each exercise:
        // slow 10 x 10 s + 9 rests of 3 s, holds 10 x (10 s + 3 s), quick 10 x 10 s + 9 rests.
        assert_eq!(
            plan().total_ms,
            15_000 + 4_000 + 127_000 + 130_000 + 127_000
        );
    }

    #[test]
    fn steps_run_back_to_back() {
        let p = plan();
        let mut t = 0;
        for s in &p.steps {
            assert_eq!(s.start_ms, t);
            t += s.duration_ms;
        }
        assert_eq!(t, p.total_ms);
    }

    #[test]
    fn each_exercise_is_announced_five_seconds_ahead() {
        let p = plan();
        let lead_ins: Vec<_> = p
            .steps
            .iter()
            .filter(|s| s.phase == Phase::GetReady)
            .collect();
        assert_eq!(lead_ins.len(), 3);
        assert!(lead_ins.iter().all(|s| s.duration_ms == 5_000));
        assert_eq!(
            p.cues[0],
            Cue {
                at_ms: 0,
                say: Say::GetReady { exercise: 1 }
            }
        );
    }

    #[test]
    fn a_slow_squeeze_is_counted_squeeze_two_three_four_five() {
        let p = plan();
        let said: Vec<_> = p
            .cues
            .iter()
            .filter(|c| (5_000..10_000).contains(&c.at_ms))
            .map(|c| c.say.clone())
            .collect();
        assert_eq!(
            said,
            vec![
                Say::Squeeze,
                Say::Count { n: 2 },
                Say::Count { n: 3 },
                Say::Count { n: 4 },
                Say::Count { n: 5 },
            ]
        );
    }

    #[test]
    fn there_is_no_rest_after_the_last_rep() {
        let p = plan();
        let last_of_first = p
            .steps
            .iter()
            .rfind(|s| s.exercise == 1 && s.rep == 10)
            .unwrap();
        assert_eq!(last_of_first.phase, Phase::Release);
    }

    #[test]
    fn the_session_ends_with_a_done_cue_at_the_very_end() {
        let p = plan();
        assert_eq!(
            p.cues.last(),
            Some(&Cue {
                at_ms: p.total_ms,
                say: Say::SessionDone
            })
        );
    }

    #[test]
    fn cues_are_in_time_order() {
        let p = plan();
        assert!(p.cues.windows(2).all(|w| w[0].at_ms <= w[1].at_ms));
    }
}
