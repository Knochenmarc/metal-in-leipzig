use std::borrow::Borrow;

use chrono::NaiveDateTime;

use crate::event::{Event, Location};
use crate::site::Site;
use crate::tools::Http;

pub struct ZeitgeschichtlichesForum<'l> {
    location: Location<'l, 'l, 'l>,
}

impl ZeitgeschichtlichesForum<'_> {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "zf",
                name: "Zeitgeschichtliches Forum",
                website: "https://www.hdg.de/zeitgeschichtliches-forum",
            },
        }
    }
}

impl Site for ZeitgeschichtlichesForum<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, _http: &Http) -> Vec<Event> {
        let mut result = Vec::new();

        // let event = Event::new(
        //     "Heavy Metal nach dem Mauerfall".to_string(),
        //     NaiveDateTime::parse_from_str("2024-02-07 19:00", "%Y-%m-%d %H:%M").unwrap(),
        //     self.location.borrow(),
        //     "https://www.hdg.de/zeitgeschichtliches-forum/veranstaltungen/heavy-metal-nach-dem-mauerfall-07-02-2024".to_string(),
        //     Some("https://www.hdg.de/fileadmin/_processed_/3/1/csm_Blackout_Live_35f325230c.jpeg".to_string()),
        // );
        // result.push(event);

        result
    }
}
