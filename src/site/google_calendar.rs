use chrono::{Months, NaiveDateTime};
use regex::Regex;
use serde_json::Value;
use urlencoding::encode;

use crate::event::{Event, Location};
use crate::tools::date::{get_today, parse_iso_date, parse_iso_datetime};
use crate::tools::Http;

fn fetch_events(http: &Http, developer_key: String, calendar_id: String) -> Value {
    let mut url = String::new();
    url.push_str("https://clients6.google.com/calendar/v3/calendars/");
    url.push_str(calendar_id.as_str());
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
    url.push_str(developer_key.as_str());

    http.get_json(url.as_str()).unwrap()
}

fn parse_date(date: Option<&Value>) -> NaiveDateTime {
    let date_struct = date.unwrap().as_object().unwrap();
    let date_time = date_struct.get("dateTime");
    if let Some(date_time) = date_time {
        return parse_iso_datetime(date_time.as_str().unwrap()).unwrap();
    }

    parse_iso_date(date_struct.get("date").unwrap().as_str().unwrap())
}

pub fn fetch_calendar_events<'a, 'b, 'c, 'd>(
    http: &'a Http,
    developer_key: &'b str,
    calendar_id: &'c str,
    location: &'d Location,
) -> Vec<Event<'d>> {
    let mut response = vec![];

    let raw_data = fetch_events(http, developer_key.to_string(), calendar_id.to_string());
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
