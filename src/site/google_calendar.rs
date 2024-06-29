use chrono::{Months, NaiveDateTime};
use regex::Regex;
use serde_json::Value;
use urlencoding::encode;

use crate::event::{Event, Location};
use crate::tools::date::{get_today, parse_iso_date, parse_iso_datetime};
use crate::tools::Http;

fn fetch_initial_config(http: &Http, source: &str) -> Value {
    let reg = Regex::new(r"(?i)window._init\((.*?)\);}</script>").unwrap();

    let mut url = "https://calendar.google.com/calendar/embed?wkst=2&ctz=Europe%2FBerlin&showDate=1&showPrint=0&showTabs=0&showCalendars=0&showTz=0&src=".to_owned();
    url.push_str(source);
    let result = http.get(url.as_str()).unwrap();

    let capture = reg.captures(result.as_str()).unwrap();
    let json = capture.get(1).unwrap().as_str();
    serde_json::from_str(json).unwrap()
}

fn fetch_events(http: &Http, config: Value) -> Value {
    let developer_key = config.get("developerKey").unwrap().as_str().unwrap();
    let calendar_id = config
        .get("cids")
        .unwrap()
        .as_object()
        .unwrap()
        .keys()
        .next()
        .unwrap();
    let mut url = config.get("proxyUrl").unwrap().as_str().unwrap().to_owned();
    url.push_str("/calendar/v3/calendars/");
    url.push_str(calendar_id);
    url.push_str("/events?singleEvents=true&timeZone=Europe%2FBerlin&maxAttendees=1&maxResults=250&sanitizeHtml=true&timeMin=");
    url.push_str(
        encode(get_today().to_rfc3339().as_str())
            .to_string()
            .as_str(),
    );
    url.push_str("&timeMax=");
    url.push_str(
        encode(
            get_today()
                .checked_add_months(Months::new(6))
                .unwrap()
                .to_rfc3339()
                .as_str(),
        )
        .to_string()
        .as_str(),
    );
    url.push_str("&key=");
    url.push_str(developer_key);

    http.get_json(url.as_str()).unwrap()
}

fn parse_date(date: Option<&Value>) -> NaiveDateTime {
    let date_struct = date.unwrap().as_object().unwrap();
    let date_time = date_struct.get("dateTime");
    if date_time.is_some() {
        return parse_iso_datetime(date_time.unwrap().as_str().unwrap()).unwrap();
    }

    parse_iso_date(date_struct.get("date").unwrap().as_str().unwrap())
}

pub fn fetch_calendar_events<'a, 'b, 'c>(
    http: &'a Http,
    source: &'b str,
    location: &'c Location,
) -> Vec<Event<'c>> {
    let config = fetch_initial_config(http, source);

    let mut response = vec![];

    let raw_data = fetch_events(http, config);
    let items = raw_data.get("items").unwrap().as_array().unwrap();
    for item in items {
        if item
            .get("visibility")
            .unwrap()
            .as_str()
            .unwrap()
            .eq("public")
        {
            let mut event = Event::new(
                item.get("summary").unwrap().as_str().unwrap().to_string(),
                parse_date(item.get("start")),
                location,
                item.get("htmlLink").unwrap().as_str().unwrap().to_string(),
                None,
            );
            event.description = item.get("description").map(|s| s.to_string());
            event.end_date = Some(parse_date(item.get("end")));
            response.push(event);
        }
    }

    response
}
