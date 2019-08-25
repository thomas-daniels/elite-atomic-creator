use chrono::prelude::*;
use time::Duration;

/// Returns the next date for the Elite Atomic after the time specified in the parameter `after`.
/// Elite Atomics happen on Sundays at 19:00 UTC.
/// If `after` is not a Sunday, it returns the next Sunday.
/// If `after` is a Sunday, it returns the same Sunday if the time is before 19:00 UTC,
/// and the next Sunday if the time is after 19:00 UTC.
fn date_of_next_tournament_after(after: DateTime<Utc>) -> Date<Utc> {
    let weekday = after.date().naive_utc().weekday();

    if weekday == Weekday::Sun {
        if after.time().hour() < 19 {
            after.date()
        } else {
            after.date() + Duration::days(7)
        }
    } else {
        after.date() + Duration::days(7_i64 - i64::from(weekday.num_days_from_sunday()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_of_next_tournament_after_weekday() {
        let after =
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2019, 8, 23).and_hms(18, 53, 00), Utc);

        assert_eq!(
            date_of_next_tournament_after(after),
            Date::<Utc>::from_utc(NaiveDate::from_ymd(2019, 8, 25), Utc)
        );
    }

    #[test]
    fn test_date_of_next_tournament_after_sunday_before_19() {
        let after =
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2019, 8, 25).and_hms(18, 53, 00), Utc);

        assert_eq!(
            date_of_next_tournament_after(after),
            Date::<Utc>::from_utc(NaiveDate::from_ymd(2019, 8, 25), Utc)
        );
    }

    #[test]
    fn test_date_of_next_tournament_after_sunday_after_19() {
        let after =
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2019, 8, 25).and_hms(19, 53, 00), Utc);

        assert_eq!(
            date_of_next_tournament_after(after),
            Date::<Utc>::from_utc(NaiveDate::from_ymd(2019, 9, 1), Utc)
        );
    }
}
