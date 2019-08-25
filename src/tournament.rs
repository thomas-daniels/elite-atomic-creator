use crate::date;

pub struct Info<'a> {
    name: &'a str,
    clock_time: i32,
    clock_increment: i32,
    minutes: i32,
    start_date: i64,
    variant: &'a str,
    rated: bool,
    berserkable: bool,
    min_rating: i32,
}

pub fn elite_atomic_at<'a>(datetime: date::UtcDateTime) -> Info<'a> {
    let time_control = [(3, 2), (1, 1), (3, 0), (1, 0), (2, 1)][date::which(datetime)];
    Info {
        name: "Elite Atomic",
        clock_time: time_control.0,
        clock_increment: time_control.1,
        minutes: 120,
        start_date: date::to_millis(datetime),
        variant: "atomic",
        rated: true,
        berserkable: true,
        min_rating: 2000,
    }
}
