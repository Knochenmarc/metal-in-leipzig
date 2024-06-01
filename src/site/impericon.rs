use std::borrow::Borrow;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::event::{Event, EventType, Location};
use crate::site::Site;
use crate::tools::Http;

pub struct Impericon<'l> {
    location: Location<'l, 'l, 'l>,
}

impl Impericon<'_> {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "imp",
                name: "Impericon Festival",
                website: "https://www.impericon.com/de/festival",
            },
        }
    }
}

impl Site for Impericon<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, _http: &Http) -> Vec<Event> {
        return vec![];

        let mut evt = Event::new(
            "Impericon Festival".to_string(),
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 3, 30).unwrap(),
                NaiveTime::default(),
            ),
            self.location.borrow(),
            "https://www.impericon.com/de/festival".to_string(),
            Some("https://www.impericon.com/1248x755x90/media/impericon/tickets/impericonfestivals2024/20231219_imp_fest_24_microseite_vo4_bands_fullsize_leipzig.jpg".to_string()),
        );
        evt.evt_type = EventType::Festival;

        vec![evt]
    }
}
