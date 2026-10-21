//! Progress: which sessions are done, streaks, pretzel points and the calendar.

use std::collections::BTreeSet;

use chrono::{Datelike, Days, NaiveDate};
use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::routine::{rotomies, Routine};

/// The two daily sessions.
#[derive(Tsify, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum Slot {
    Morning,
    Afternoon,
}

/// Which slot a session started at this hour (0-23) counts for.
pub fn slot_for_hour(hour: u32) -> Slot {
    if hour < 13 {
        Slot::Morning
    } else {
        Slot::Afternoon
    }
}

/// A session the user did, either with the coach or marked done by hand.
#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SessionRecord {
    #[tsify(type = "string")]
    pub date: NaiveDate,
    pub slot: Slot,
    pub marked_by_hand: bool,
}

pub const POINTS_PER_SESSION: u32 = 10;
/// Extra points for doing both sessions on one day.
pub const FULL_DAY_BONUS: u32 = 5;
pub const JAR_CAPACITY: u32 = 100;

#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DayProgress {
    #[tsify(type = "string")]
    pub date: NaiveDate,
    pub morning: bool,
    pub afternoon: bool,
}

#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProgressSummary {
    pub today: DayProgress,
    /// Days in a row with at least one session. Today still counts as "in a row" until it ends.
    pub streak_days: u32,
    pub sessions: u32,
    pub pretzel_points: u32,
    pub jars_filled: u32,
    /// Points in the jar that is filling now, out of `jar_capacity`.
    pub jar_points: u32,
    pub jar_capacity: u32,
    pub rotomies: u32,
}

/// Each (day, slot) counts once, however many times it was recorded.
fn done_slots(records: &[SessionRecord]) -> BTreeSet<(NaiveDate, Slot)> {
    records.iter().map(|r| (r.date, r.slot)).collect()
}

fn day(done: &BTreeSet<(NaiveDate, Slot)>, date: NaiveDate) -> DayProgress {
    DayProgress {
        date,
        morning: done.contains(&(date, Slot::Morning)),
        afternoon: done.contains(&(date, Slot::Afternoon)),
    }
}

fn streak(done: &BTreeSet<(NaiveDate, Slot)>, today: NaiveDate) -> u32 {
    let days: BTreeSet<NaiveDate> = done.iter().map(|(d, _)| *d).collect();
    // A missed day only breaks the streak once it is over.
    let mut cursor = if days.contains(&today) {
        today
    } else {
        match today.checked_sub_days(Days::new(1)) {
            Some(d) => d,
            None => return 0,
        }
    };
    let mut streak = 0;
    while days.contains(&cursor) {
        streak += 1;
        match cursor.checked_sub_days(Days::new(1)) {
            Some(d) => cursor = d,
            None => break,
        }
    }
    streak
}

pub fn progress_summary(
    records: &[SessionRecord],
    today: NaiveDate,
    routine: &Routine,
) -> ProgressSummary {
    let done = done_slots(records);
    let sessions = done.len() as u32;
    let full_days = done
        .iter()
        .filter(|(d, s)| *s == Slot::Morning && done.contains(&(*d, Slot::Afternoon)))
        .count() as u32;
    let points = sessions * POINTS_PER_SESSION + full_days * FULL_DAY_BONUS;

    ProgressSummary {
        today: day(&done, today),
        streak_days: streak(&done, today),
        sessions,
        pretzel_points: points,
        jars_filled: points / JAR_CAPACITY,
        jar_points: points % JAR_CAPACITY,
        jar_capacity: JAR_CAPACITY,
        rotomies: sessions * rotomies(routine),
    }
}

#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CalendarMonth {
    pub year: i32,
    /// 1-12.
    pub month: u32,
    /// Empty cells before the 1st, in a week that starts on Sunday.
    pub leading_blanks: u32,
    pub days: Vec<DayProgress>,
}

/// One month of the calendar, or None for a month that doesn't exist.
pub fn calendar_month(records: &[SessionRecord], year: i32, month: u32) -> Option<CalendarMonth> {
    let first = NaiveDate::from_ymd_opt(year, month, 1)?;
    let done = done_slots(records);
    let days = first
        .iter_days()
        .take_while(|d| d.month() == month)
        .map(|d| day(&done, d))
        .collect();
    Some(CalendarMonth {
        year,
        month,
        leading_blanks: first.weekday().num_days_from_sunday(),
        days,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routine::default_routine;

    fn date(s: &str) -> NaiveDate {
        s.parse().unwrap()
    }

    fn rec(d: &str, slot: Slot) -> SessionRecord {
        SessionRecord {
            date: date(d),
            slot,
            marked_by_hand: false,
        }
    }

    fn summary(records: &[SessionRecord], today: &str) -> ProgressSummary {
        progress_summary(records, date(today), &default_routine())
    }

    #[test]
    fn mornings_end_at_one_pm() {
        assert_eq!(slot_for_hour(9), Slot::Morning);
        assert_eq!(slot_for_hour(12), Slot::Morning);
        assert_eq!(slot_for_hour(13), Slot::Afternoon);
        assert_eq!(slot_for_hour(16), Slot::Afternoon);
    }

    #[test]
    fn two_days_in_a_row_is_a_streak_of_two() {
        let r = [
            rec("2026-10-20", Slot::Morning),
            rec("2026-10-21", Slot::Afternoon),
        ];
        assert_eq!(summary(&r, "2026-10-21").streak_days, 2);
    }

    #[test]
    fn todays_streak_survives_until_the_day_is_over() {
        let r = [
            rec("2026-10-19", Slot::Morning),
            rec("2026-10-20", Slot::Morning),
        ];
        assert_eq!(summary(&r, "2026-10-21").streak_days, 2);
        assert_eq!(summary(&r, "2026-10-22").streak_days, 0);
    }

    #[test]
    fn a_session_recorded_twice_counts_once() {
        let r = [
            rec("2026-10-21", Slot::Morning),
            rec("2026-10-21", Slot::Morning),
        ];
        let s = summary(&r, "2026-10-21");
        assert_eq!(s.sessions, 1);
        assert_eq!(s.pretzel_points, POINTS_PER_SESSION);
    }

    #[test]
    fn doing_both_sessions_in_a_day_earns_a_bonus() {
        let r = [
            rec("2026-10-21", Slot::Morning),
            rec("2026-10-21", Slot::Afternoon),
        ];
        let s = summary(&r, "2026-10-21");
        assert_eq!(s.pretzel_points, 2 * POINTS_PER_SESSION + FULL_DAY_BONUS);
        assert!(s.today.morning && s.today.afternoon);
    }

    #[test]
    fn the_jar_empties_into_a_new_one_when_full() {
        // 5 full days = 5 * 25 = 125 points
        let r: Vec<_> = (1..=5)
            .flat_map(|d| {
                let d = format!("2026-10-0{d}");
                [rec(&d, Slot::Morning), rec(&d, Slot::Afternoon)]
            })
            .collect();
        let s = summary(&r, "2026-10-05");
        assert_eq!((s.jars_filled, s.jar_points), (1, 25));
    }

    #[test]
    fn every_session_adds_its_rotomies() {
        let r = [rec("2026-10-21", Slot::Morning)];
        assert_eq!(summary(&r, "2026-10-21").rotomies, 120);
    }

    #[test]
    fn october_2026_starts_on_a_thursday() {
        let m = calendar_month(&[rec("2026-10-21", Slot::Afternoon)], 2026, 10).unwrap();
        assert_eq!(m.leading_blanks, 4);
        assert_eq!(m.days.len(), 31);
        assert!(m.days[20].afternoon && !m.days[20].morning);
    }

    #[test]
    fn there_is_no_thirteenth_month() {
        assert_eq!(calendar_month(&[], 2026, 13), None);
    }
}
