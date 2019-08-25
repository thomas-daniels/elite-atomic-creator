use chrono::prelude::*;
use time::Duration;

/// This type has two purposes:
/// * Avoiding to type the generics every time.
/// * The other modules can use UtcDateTime without having to care
///   about what the underlying type is, so the logic and crates used in
///   date.rs can be changed without touching other modules.
pub type UtcDateTime = DateTime<Utc>;

/// Returns the next date for the Elite Atomic after the time specified in the parameter `after`.
/// Elite Atomics happen on Sundays at 19:00 UTC.
/// If `after` is not a Sunday, it returns the next Sunday.
/// If `after` is a Sunday, it returns the same Sunday if the time is before 19:00 UTC,
/// and the next Sunday if the time is after 19:00 UTC.
pub fn datetime_of_next_tournament_after(after: &UtcDateTime) -> UtcDateTime {
    let weekday = after.date().naive_utc().weekday();

    let date = if weekday == Weekday::Sun {
        if after.time().hour() < 19 {
            after.date()
        } else {
            after.date() + Duration::days(7)
        }
    } else {
        after.date() + Duration::days(7_i64 - i64::from(weekday.num_days_from_sunday()))
    };
    date.and_hms(19, 0, 0)
}

/// Returns the ordinal rank for a tournament played on `date`.
/// Meaning, returns 0 if the first tournament of the month happens on `date`,
/// 1 if the second tournament of the month happens on `date`, etc.
pub fn which(date: &UtcDateTime) -> usize {
    (date.day() - 1) as usize / 7
}

/// Transforms a date into the format that the Lichess API wants,
/// i.e. the number of (non-leap) milliseconds since January 1, 1970 UTC.
///
/// This function is just a single call but I'm still making a function
/// for it here to keep all the date-related logic in one place.
pub fn to_millis(datetime: &UtcDateTime) -> i64 {
    datetime.timestamp_millis()
}

/// Returns an UtcDateTime for the current time.
///
/// Again a single call just like to_millis, with the same idea
/// to isolate all date-related logic.
pub fn now() -> UtcDateTime {
    Utc::now()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_of_next_tournament_after_weekday() {
        let after =
            UtcDateTime::from_utc(NaiveDate::from_ymd(2019, 8, 23).and_hms(18, 53, 00), Utc);

        assert_eq!(
            datetime_of_next_tournament_after(&after),
            UtcDateTime::from_utc(NaiveDate::from_ymd(2019, 8, 25).and_hms(19, 0, 0), Utc)
        );
    }

    #[test]
    fn test_date_of_next_tournament_after_sunday_before_19() {
        let after =
            UtcDateTime::from_utc(NaiveDate::from_ymd(2019, 8, 25).and_hms(18, 53, 00), Utc);

        assert_eq!(
            datetime_of_next_tournament_after(&after),
            UtcDateTime::from_utc(NaiveDate::from_ymd(2019, 8, 25).and_hms(19, 0, 0), Utc)
        );
    }

    #[test]
    fn test_date_of_next_tournament_after_sunday_after_19() {
        let after =
            UtcDateTime::from_utc(NaiveDate::from_ymd(2019, 8, 25).and_hms(19, 53, 00), Utc);

        assert_eq!(
            datetime_of_next_tournament_after(&after),
            UtcDateTime::from_utc(NaiveDate::from_ymd(2019, 9, 1).and_hms(19, 0, 0), Utc)
        );
    }

    #[test]
    fn test_which_0() {
        assert_eq!(
            which(&UtcDateTime::from_utc(
                NaiveDate::from_ymd(2019, 7, 7).and_hms(19, 0, 0),
                Utc
            )),
            0
        );
        assert_eq!(
            which(&UtcDateTime::from_utc(
                NaiveDate::from_ymd(2019, 8, 4).and_hms(19, 0, 0),
                Utc
            )),
            0
        );
        assert_eq!(
            which(&UtcDateTime::from_utc(
                NaiveDate::from_ymd(2019, 9, 1).and_hms(19, 0, 0),
                Utc
            )),
            0
        );
    }

    #[test]
    fn test_which_1() {
        assert_eq!(
            which(&UtcDateTime::from_utc(
                NaiveDate::from_ymd(2019, 7, 14).and_hms(19, 0, 0),
                Utc
            )),
            1
        );
        assert_eq!(
            which(&UtcDateTime::from_utc(
                NaiveDate::from_ymd(2019, 8, 11).and_hms(19, 0, 0),
                Utc
            )),
            1
        );
        assert_eq!(
            which(&UtcDateTime::from_utc(
                NaiveDate::from_ymd(2019, 9, 8).and_hms(19, 0, 0),
                Utc
            )),
            1
        );
    }

    #[test]
    fn test_which_2() {
        assert_eq!(
            which(&UtcDateTime::from_utc(
                NaiveDate::from_ymd(2019, 7, 21).and_hms(19, 0, 0),
                Utc
            )),
            2
        );
        assert_eq!(
            which(&UtcDateTime::from_utc(
                NaiveDate::from_ymd(2019, 8, 18).and_hms(19, 0, 0),
                Utc
            )),
            2
        );
        assert_eq!(
            which(&UtcDateTime::from_utc(
                NaiveDate::from_ymd(2019, 9, 15).and_hms(19, 0, 0),
                Utc
            )),
            2
        );
    }

    #[test]
    fn test_which_3() {
        assert_eq!(
            which(&UtcDateTime::from_utc(
                NaiveDate::from_ymd(2019, 7, 28).and_hms(19, 0, 0),
                Utc
            )),
            3
        );
        assert_eq!(
            which(&UtcDateTime::from_utc(
                NaiveDate::from_ymd(2019, 8, 25).and_hms(19, 0, 0),
                Utc
            )),
            3
        );
        assert_eq!(
            which(&UtcDateTime::from_utc(
                NaiveDate::from_ymd(2019, 9, 22).and_hms(19, 0, 0),
                Utc
            )),
            3
        );
    }

    #[test]
    fn test_which_4() {
        assert_eq!(
            which(&UtcDateTime::from_utc(
                NaiveDate::from_ymd(2019, 9, 29).and_hms(19, 0, 0),
                Utc
            )),
            4
        );
    }
}
