use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use crate::event::{Event, Location};
use crate::tools::date::parse_icalendar_datetime;
use crate::tools::Http;

fn parse_ics_events(data: String) -> Vec<HashMap<String, String>> {
    let mut result = vec![];

    let data = data.replace("\n ", " ");
    let mut current_event: Option<HashMap<String, String>> = None;
    for line in data.lines() {
        if line == "BEGIN:VEVENT" {
            current_event = Some(HashMap::new());
        } else if line == "END:VEVENT" {
            if let Some(event) = current_event {
                result.push(event);
            }
            current_event = None
        } else if current_event.is_some() {
            let mut map = current_event.unwrap();

            if let Some((key, value)) = line.split_once(':') {
                map.insert(key.to_string(), value.to_string());
            }

            current_event = Some(map);
        }
    }

    result
}

fn fetch_image(http: &Http, event_url: &String) -> Option<String> {
    lazy_static! {
        static ref REG: Regex = Regex::new("\"full_image\":\\{.*?\"uri\":\"(.*?)\".*?\\}").unwrap();
    }

    let response = http.get(event_url.as_str()).unwrap();

    REG.captures(response.as_str())
        .unwrap()
        .get(1)
        .map(|m| m.as_str().replace("\\/", "/"))
}

pub fn fetch_facebook_events<'a, 'b>(
    http: &'a Http,
    location: &'b Location,
    page_id: u64,
    key: String,
) -> Vec<Event<'b>> {
    let mut result = vec![];

    let response = http
        .get(
            format!(
                "https://www.facebook.com/events/ical/upcoming/?uid={}&key={}",
                page_id, key
            )
            .as_str(),
        )
        .unwrap();

    for ics_event in parse_ics_events(response) {
        let mut event = Event::new(
            ics_event.get("SUMMARY").unwrap().clone(),
            parse_icalendar_datetime(ics_event.get("DTSTART").unwrap().as_str()).unwrap(),
            location,
            ics_event.get("URL").unwrap().clone(),
            fetch_image(http, ics_event.get("URL").unwrap()),
        );
        event.description = ics_event.get("DESCRIPTION").cloned();
        event.end_date =
            Some(parse_icalendar_datetime(ics_event.get("DTEND").unwrap().as_str()).unwrap());
        result.push(event);
    }

    result
}
