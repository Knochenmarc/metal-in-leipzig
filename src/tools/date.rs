use chrono::NaiveDate;

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
