use crate::event::{Event, EventType, Location};
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::{Map, Value};

use crate::site::parse_linked_data_events;
use crate::tools::date::parse_iso_datetime;
use crate::tools::Http;

pub fn fetch_tixforgigs_events<'l>(
    http: &Http,
    location_id: &str,
    location: &'l Location,
) -> Vec<Event<'l>> {
    lazy_static! {
        static ref REG: Regex = Regex::new(r"vm.setLocationId\(\d+,(\{.+\})\)").unwrap();
    }

    let mut result = Vec::new();

    let response = http
        .get(format!("https://www.tixforgigs.com/de-de/Location/{}", location_id).as_str())
        .unwrap();

    let json_str = REG
        .captures_iter(&response)
        .last()
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string();
    let doc: Value = serde_json::from_str(json_str.as_str()).unwrap();
    let json = doc.as_object().unwrap();
    let json_events = json.get("futureEvents").unwrap().as_array().unwrap();
    for json_event in json_events {
        let json_event = json_event.as_object().unwrap();
        let event_id = json_event.get("eventId").unwrap().as_i64().unwrap();
        let event_data = fetch_tixforgigs_event(http, event_id.to_string().as_str());

        if event_data.is_some() {
            let event_data = event_data.unwrap();
            let mut event = Event::new(
                event_data
                    .get("name")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
                parse_iso_datetime(event_data.get("startDate").unwrap().as_str().unwrap()).unwrap(),
                location,
                format!("https://www.tixforgigs.com/de-de/Event/{}", event_id),
                Some(
                    event_data
                        .get("image")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string(),
                ),
            );
            event.evt_type = EventType::Concert;
            result.push(event);
        }
    }

    result
}

pub fn fetch_tixforgigs_event(http: &Http, event_id: &str) -> Option<Map<String, Value>> {
    lazy_static! {
        static ref REG: Regex =
            Regex::new("<link rel=\"preload\" as=\"image\" href=\"(.*?)\" />").unwrap();
    }

    let response = http
        .get(format!("https://www.tixforgigs.com/de-de/Event/{}", event_id).as_str())
        .unwrap();

    let events = parse_linked_data_events(&response);
    if events.is_empty() {
        return None;
    }

    let image = REG
        .captures_iter(&response)
        .last()
        .map(|s| s.get(1).unwrap().as_str().to_string());

    let mut value = events.first().unwrap().clone();

    if let Some(image) = image {
        if !image.contains("notAvailable") {
            value["image"] = Value::String(image);
        }
    }

    Some(value.as_object().unwrap().to_owned())
}
