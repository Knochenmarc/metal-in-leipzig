use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

use crate::site::parse_linked_data_events;
use crate::tools::Http;

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
