use chrono::prelude::*;
use time::Duration;

/// Returns the next date for the Elite Atomic after the time specified in the parameter `after`.
/// Elite Atomics happen on Sundays at 19:00 UTC.
/// If `after` is not a Sunday, it returns the next Sunday.
/// If `after` is a Sunday, it returns the same Sunday if the time is before 19:00 UTC,
/// and the next Sunday if the time is after 19:00 UTC.
fn datetime_of_next_tournament_after(after: DateTime<Utc>) -> DateTime<Utc> {
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
fn which(date: Date<Utc>) -> usize {
    (date.day() - 1) as usize / 7
}

/// Transforms a date into the format that the Lichess API wants,
/// i.e. the number of (non-leap) milliseconds since January 1, 1970 UTC.
///
/// This function is just a single call but I'm still making a function
/// for it here to keep all the date-related logic in one place.
fn to_millis(datetime: DateTime<Utc>) -> i64 {
    datetime.timestamp_millis()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_of_next_tournament_after_weekday() {
        let after =
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2019, 8, 23).and_hms(18, 53, 00), Utc);

        assert_eq!(
            datetime_of_next_tournament_after(after),
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2019, 8, 25).and_hms(19, 0, 0), Utc)
        );
    }

    #[test]
    fn test_date_of_next_tournament_after_sunday_before_19() {
        let after =
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2019, 8, 25).and_hms(18, 53, 00), Utc);

        assert_eq!(
            datetime_of_next_tournament_after(after),
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2019, 8, 25).and_hms(19, 0, 0), Utc)
        );
    }

    #[test]
    fn test_date_of_next_tournament_after_sunday_after_19() {
        let after =
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2019, 8, 25).and_hms(19, 53, 00), Utc);

        assert_eq!(
            datetime_of_next_tournament_after(after),
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2019, 9, 1).and_hms(19, 0, 0), Utc)
        );
    }

    #[test]
    fn test_which_0() {
        assert_eq!(
            which(Date::<Utc>::from_utc(NaiveDate::from_ymd(2019, 7, 7), Utc)),
            0
        );
        assert_eq!(
            which(Date::<Utc>::from_utc(NaiveDate::from_ymd(2019, 8, 4), Utc)),
            0
        );
        assert_eq!(
            which(Date::<Utc>::from_utc(NaiveDate::from_ymd(2019, 9, 1), Utc)),
            0
        );
    }

    #[test]
    fn test_which_1() {
        assert_eq!(
            which(Date::<Utc>::from_utc(NaiveDate::from_ymd(2019, 7, 14), Utc)),
            1
        );
        assert_eq!(
            which(Date::<Utc>::from_utc(NaiveDate::from_ymd(2019, 8, 11), Utc)),
            1
        );
        assert_eq!(
            which(Date::<Utc>::from_utc(NaiveDate::from_ymd(2019, 9, 8), Utc)),
            1
        );
    }

    #[test]
    fn test_which_2() {
        assert_eq!(
            which(Date::<Utc>::from_utc(NaiveDate::from_ymd(2019, 7, 21), Utc)),
            2
        );
        assert_eq!(
            which(Date::<Utc>::from_utc(NaiveDate::from_ymd(2019, 8, 18), Utc)),
            2
        );
        assert_eq!(
            which(Date::<Utc>::from_utc(NaiveDate::from_ymd(2019, 9, 15), Utc)),
            2
        );
    }

    #[test]
    fn test_which_3() {
        assert_eq!(
            which(Date::<Utc>::from_utc(NaiveDate::from_ymd(2019, 7, 28), Utc)),
            3
        );
        assert_eq!(
            which(Date::<Utc>::from_utc(NaiveDate::from_ymd(2019, 8, 25), Utc)),
            3
        );
        assert_eq!(
            which(Date::<Utc>::from_utc(NaiveDate::from_ymd(2019, 9, 22), Utc)),
            3
        );
    }

    #[test]
    fn test_which_4() {
        assert_eq!(
            which(Date::<Utc>::from_utc(NaiveDate::from_ymd(2019, 9, 29), Utc)),
            4
        );
    }
}
