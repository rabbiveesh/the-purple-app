//! When the weekly and monthly check-in questions come round.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::progress::SessionRecord;

pub const WEEKLY_EVERY_DAYS: i64 = 7;
pub const MONTHLY_EVERY_DAYS: i64 = 28;

#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CheckInHistory {
    pub records: Vec<SessionRecord>,
    #[tsify(type = "string | undefined", optional)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_weekly: Option<NaiveDate>,
    #[tsify(type = "string | undefined", optional)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_monthly: Option<NaiveDate>,
    #[tsify(type = "string")]
    pub today: NaiveDate,
}

#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CheckInsDue {
    pub weekly: bool,
    pub monthly: bool,
}

fn due(last: Option<NaiveDate>, today: NaiveDate, every_days: i64) -> bool {
    last.is_none_or(|d| (today - d).num_days() >= every_days)
}

pub fn check_ins_due(h: &CheckInHistory) -> CheckInsDue {
    let Some(first) = h.records.iter().map(|r| r.date).min() else {
        // Nothing to compare against before the first session.
        return CheckInsDue {
            weekly: false,
            monthly: false,
        };
    };
    CheckInsDue {
        weekly: due(h.last_weekly, h.today, WEEKLY_EVERY_DAYS),
        monthly: (h.today - first).num_days() >= MONTHLY_EVERY_DAYS
            && due(h.last_monthly, h.today, MONTHLY_EVERY_DAYS),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::progress::Slot;

    fn date(s: &str) -> NaiveDate {
        s.parse().unwrap()
    }

    fn history(
        first: &str,
        weekly: Option<&str>,
        monthly: Option<&str>,
        today: &str,
    ) -> CheckInHistory {
        CheckInHistory {
            records: vec![SessionRecord {
                date: date(first),
                slot: Slot::Morning,
                marked_by_hand: false,
            }],
            last_weekly: weekly.map(date),
            last_monthly: monthly.map(date),
            today: date(today),
        }
    }

    #[test]
    fn nothing_is_asked_before_the_first_session() {
        let h = CheckInHistory {
            records: vec![],
            last_weekly: None,
            last_monthly: None,
            today: date("2026-10-21"),
        };
        assert_eq!(
            check_ins_due(&h),
            CheckInsDue {
                weekly: false,
                monthly: false
            }
        );
    }

    #[test]
    fn the_weekly_check_in_comes_a_week_after_the_last_one() {
        assert!(
            !check_ins_due(&history(
                "2026-10-01",
                Some("2026-10-15"),
                None,
                "2026-10-21"
            ))
            .weekly
        );
        assert!(
            check_ins_due(&history(
                "2026-10-01",
                Some("2026-10-14"),
                None,
                "2026-10-21"
            ))
            .weekly
        );
    }

    #[test]
    fn the_monthly_question_waits_four_weeks_from_the_start() {
        assert!(!check_ins_due(&history("2026-10-01", None, None, "2026-10-28")).monthly);
        assert!(check_ins_due(&history("2026-10-01", None, None, "2026-10-29")).monthly);
        assert!(
            !check_ins_due(&history(
                "2026-09-01",
                None,
                Some("2026-10-10"),
                "2026-10-29"
            ))
            .monthly
        );
    }
}
