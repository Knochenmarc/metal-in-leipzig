use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, ParseResult, Utc};
use chrono_tz::Tz;
use std::time::SystemTime;

pub fn get_today() -> DateTime<Tz> {
    let now: DateTime<Utc> = SystemTime::now().into();
    let today = now.date_naive().and_hms_opt(0, 0, 0).unwrap();
    today.and_local_timezone(chrono_tz::Europe::Berlin).unwrap()
}

pub fn parse_english_date(str: &str) -> NaiveDate {
    NaiveDate::parse_from_str(str, "%d %B %Y").unwrap()
}

pub fn parse_german_date(str: &str) -> NaiveDateTime {
    let mut str = str.replace(" Januar ", "01.");
    str = str.replace("Februar", "02");
    str = str.replace("März", "03");
    str = str.replace("April", "04");
    str = str.replace("Mai", "05");
    str = str.replace("Juni", "06");
    str = str.replace("Juli", "07");
    str = str.replace("August", "08");
    str = str.replace("September", "09");
    str = str.replace("Oktober", "10");
    str = str.replace("November", "11");
    str = str.replace("Dezember", "12");
    str = str.replace("Jan", "01");
    str = str.replace("Feb", "02");
    str = str.replace("Mär", "03");
    str = str.replace("Mrz", "03");
    str = str.replace("Apr", "04");
    str = str.replace("Mai", "05");
    str = str.replace("Jun", "06");
    str = str.replace("Jul", "07");
    str = str.replace("Aug", "08");
    str = str.replace("Sep", "09");
    str = str.replace("Okt", "10");
    str = str.replace("Nov", "11");
    str = str.replace("Dez", "12");
    str = str.replace(" ", ".");
    str = str.replace("..", ".");

    parse_short_date(str.as_str())
}

pub fn parse_short_date(str: &str) -> NaiveDateTime {
    let target = if str.len() == 6 {
        format!("{}{}", str, get_today().year())
    } else {
        str.to_string()
    };
    NaiveDate::parse_from_str(target.as_str(), "%d.%m.%Y")
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
