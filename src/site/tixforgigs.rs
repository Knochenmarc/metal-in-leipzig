use crate::event::{Event, EventType, Location};
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

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

    result
}

pub fn fetch_tixforgigs_event(http: &Http, event_id: &str) -> Value {
    lazy_static! {
        static ref REG: Regex =
            Regex::new("<link rel=\"preload\" as=\"image\" href=\"(.*?)\" />").unwrap();
    }

    let response = http
        .get(format!("https://www.tixforgigs.com/de-de/Event/{}", event_id).as_str())
        .unwrap();

    let image = REG
        .captures_iter(&response)
        .last()
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string();

    let events = parse_linked_data_events(&response);
    let mut value = events.first().unwrap().clone();
    value["image"] = Value::String(image);

    value
}
