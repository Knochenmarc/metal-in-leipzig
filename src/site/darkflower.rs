use std::borrow::Borrow;

use chrono::{DateTime, Months};

use crate::event::{Event, Location};
use crate::site::Site;
use crate::tools::date::get_today;
use crate::tools::Http;

pub struct Darkflower<'l> {
    location: Location<'l, 'l, 'l>,
}

impl Darkflower<'_> {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "df",
                name: "Darkflower",
                website: "https://darkflower.club/",
            },
        }
    }
}

impl Site for Darkflower<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut result = Vec::new();
        let today = get_today();
        let next_month = today.checked_add_months(Months::new(1)).unwrap();

        // squarespace api
        let url = "https://www.darkflower.de/api/open/GetItemsByMonth?collectionId=65eafdcbb10de026acca412e&month=";

        let responses = [
            http.get_json(format!("{}{}", url, today.format("%m-%Y")).as_str()),
            http.get_json(format!("{}{}", url, next_month.format("%m-%Y")).as_str()),
        ];

        for response in responses {
            let json = response.unwrap();
            let list = json.as_array().unwrap();
            for item in list {
                let item = item.as_object().unwrap();
                let start_date = DateTime::from_timestamp_millis(
                    item.get("startDate").unwrap().as_i64().unwrap(),
                )
                .unwrap();
                let end_date =
                    DateTime::from_timestamp_millis(item.get("endDate").unwrap().as_i64().unwrap())
                        .unwrap();
                let mut event = Event::new(
                    item.get("title").unwrap().to_string().trim().to_string(),
                    start_date.naive_local(),
                    self.get_location(),
                    format!("https://www.darkflower.de{}", item.get("fullUrl").unwrap()),
                    Some(item.get("assetUrl").unwrap().as_str().unwrap().to_string()),
                );
                event.end_date = Some(end_date.naive_local());

                let tags: Vec<String> = item
                    .get("tags")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|s| s.as_str().unwrap().to_string().to_lowercase())
                    .collect();

                if tags.is_empty() || tags.contains(&"metal".to_string()) {
                    result.push(event);
                }
            }
        }

        result
    }
}
