use chrono::{DateTime, NaiveDate, NaiveDateTime, ParseResult, Utc};
use chrono_tz::Tz;
use std::time::SystemTime;

pub fn get_today() -> DateTime<Tz> {
    let now: DateTime<Utc> = SystemTime::now().into();
    let today = now.date_naive().and_hms_opt(0, 0, 0).unwrap();
    today.and_local_timezone(chrono_tz::Europe::Berlin).unwrap()
}

pub fn parse_german_date(str: &str) -> NaiveDate {
    let mut str = str.replace(" Januar ", " January ");
    str = str.replace(" Februar ", " February ");
    str = str.replace(" März ", " March ");
    str = str.replace(" Mai ", " May ");
    str = str.replace(" Juni ", " June ");
    str = str.replace(" Juli ", " July ");
    str = str.replace(" Oktober ", " October ");
    str = str.replace(" Dezember ", " December ");
    str = str.replace(" Jan ", " January ");
    str = str.replace(" Feb ", " February ");
    str = str.replace(" Mär ", " March ");
    str = str.replace(" Mrz ", " March ");
    str = str.replace(" Apr ", " April ");
    str = str.replace(" Jun ", " June ");
    str = str.replace(" Jul ", " July ");
    str = str.replace(" Aug ", " August ");
    str = str.replace(" Sep ", " September ");
    str = str.replace(" Okt ", " October ");
    str = str.replace(" Nov ", " November ");
    str = str.replace(" Dez ", " December ");

    let format = if str.contains('.') {
        "%d. %B %Y"
    } else {
        "%d %B %Y"
    };
    NaiveDate::parse_from_str(str.as_str(), format).unwrap()
}

pub fn parse_short_date(str: &str) -> NaiveDateTime {
    NaiveDate::parse_from_str(str, "%d.%m.%Y")
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
}

pub fn parse_iso_date(str: &str) -> NaiveDateTime {
    NaiveDate::parse_from_str(str, "%Y-%m-%d")
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
}

pub fn parse_iso_datetime(str: &str) -> ParseResult<NaiveDateTime> {
    let date = str
        .to_string()
        .replace("+0:00", "+00:00")
        .replace("+1:00", "+01:00")
        .replace("+2:00", "+02:00")
        .replace("+3:00", "+03:00")
        .replace(".000", ""); // skip microseconds
    let format = match date.len() {
        19 => "%Y-%m-%dT%H:%M:%S",
        25 => "%Y-%m-%dT%H:%M:%S%:z",
        _ => "%Y-%m-%dT%H:%M%:z",
    };
    NaiveDateTime::parse_from_str(date.as_str(), format)
}
