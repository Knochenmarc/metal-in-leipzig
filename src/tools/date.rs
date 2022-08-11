use chrono::{NaiveDate, NaiveDateTime};

pub fn parse_german_date(str: &str) -> NaiveDate {
    let mut str = str.replace(" Januar ", " January ");
    str = str.replace(" Februar ", " February ");
    str = str.replace(" März ", " March ");
    str = str.replace(" Mai ", " May ");
    str = str.replace(" Juni ", " June ");
    str = str.replace(" Juli ", " July ");
    str = str.replace(" Oktober ", " October ");
    str = str.replace(" Dezember ", " December ");

    NaiveDate::parse_from_str(str.as_str(), "%d. %B %Y").unwrap()
}

pub fn parse_short_date(str: &str) -> NaiveDate {
    NaiveDate::parse_from_str(str, "%d.%m.%Y").unwrap()
}

pub fn parse_iso_datetime(str: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(
        str.to_string()
            .replace("+0:00", "+00:00")
            .replace("+1:00", "+01:00")
            .replace("+2:00", "+02:00")
            .replace("+3:00", "+03:00")
            .as_str(),
        "%Y-%m-%dT%H:%M%:z",
    )
    .unwrap()
}
