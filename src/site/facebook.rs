use chrono::DateTime;
use lazy_static::lazy_static;
use regex::Regex;

use crate::event::{Event, Location};
use crate::tools::Http;

fn decode_unicode(input: &str) -> String {
    serde_json::from_str(&format!("\"{}\"", input)).unwrap()
}

fn fetch_event<'e>(http: &Http, event_id: &str, location: &'e Location) -> Event<'e> {
    lazy_static! {
        static ref TITLE_REG: Regex = Regex::new("\"title\":\"(.*?)\",").unwrap();
        static ref IMAGE_REG: Regex = Regex::new("\"full_image\":.*?\"uri\":\"(.*?)\"").unwrap();
        static ref TIME_REG: Regex = Regex::new("\"start_timestamp\":(\\d+),").unwrap();
        static ref DESC_REG: Regex =
            Regex::new("\"event_description\":\\{\"text\":\"(.*?)\",\"").unwrap();
    }

    let url = format!("https://www.facebook.com/events/{}", event_id);
    let response = http.get(url.as_str()).unwrap();

    let title = decode_unicode(
        TITLE_REG
            .captures(&response)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str(),
    );
    let image = IMAGE_REG
        .captures(&response)
        .unwrap()
        .get(1)
        .map(|m| m.as_str().replace("\\/", "/"));
    let timestamp: i64 = TIME_REG
        .captures(&response)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    let description = decode_unicode(
        DESC_REG
            .captures(&response)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str(),
    );

    let mut event = Event::new(
        title,
        DateTime::from_timestamp(timestamp, 0).unwrap().naive_utc(),
        location,
        url,
        image,
    );
    event.description = Some(description);
    event
}

pub fn fetch_facebook_events<'e>(
    http: &Http,
    location: &'e Location,
    user_name: &str,
) -> Vec<Event<'e>> {
    let mut result = vec![];

    let response = http
        .get(
            format!(
                "https://www.facebook.com/{}/upcoming_hosted_events",
                user_name
            )
            .as_str(),
        )
        .unwrap();

    lazy_static! {
        static ref EVENT_REG: Regex =
            Regex::new("\"__typename\":\"Event\",\"id\":\"(\\d+?)\",").unwrap();
    }

    for cap in EVENT_REG.captures_iter(response.as_str()) {
        let event_id = cap.get(1).unwrap().as_str();
        result.push(fetch_event(http, event_id, location));
    }

    result
}
