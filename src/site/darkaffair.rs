use std::borrow::Borrow;

use chrono::NaiveDateTime;

use crate::event::{Event, EventType, Location};
use crate::site::Site;
use crate::tools::Http;

pub struct DarkAffair<'l> {
    location: Location<'l, 'l, 'l>,
}

impl DarkAffair<'_> {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "da",
                name: "Dark Affair",
                website: "https://www.dark-affair.com/de/",
            },
        }
    }
}

impl Site for DarkAffair<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, _http: &Http) -> Vec<Event> {
        return vec![];

        let mut event = Event::new(
            "Wolfstavar".to_string(),
            NaiveDateTime::parse_from_str("2024-05-17 20:00", "%Y-%m-%d %H:%M").unwrap(),
            self.location.borrow(),
            "https://www.dark-affair.com/de/timetable-freitag".to_string(),
            Some("https://www.dark-affair.com/data/downloads/2024/dark-affair-messe-2024-banner-390x120.jpg".to_string()),
        );
        event.evt_type = EventType::Concert;

        vec![event]
    }
}
