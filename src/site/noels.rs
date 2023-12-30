use std::borrow::Borrow;

use crate::event::{Event, Location};
use crate::site::eventim::Eventim;
use crate::site::Site;
use crate::tools::date::parse_iso_datetime;
use crate::tools::Http;

pub struct NoelsBallroom<'l> {
    location: Location<'l, 'l, 'l>,
}

impl NoelsBallroom<'_> {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "noel",
                name: "Noels Ballroom",
                website: "https://noels-ballroom.de/",
            },
        }
    }
}

impl Site for NoelsBallroom<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut result = Vec::new();
        let eventim = Eventim::new("noels-ballroom-leipzig-22594", http);

        for raw in &eventim.get_raw() {
            let mut evt = Event::new(
                raw["name"].as_str().unwrap().to_string(),
                parse_iso_datetime(raw["startDate"].as_str().unwrap()).unwrap(),
                self.location.borrow(),
                "https://noels-ballroom.de/".to_string(),
                Option::Some(raw["image"][0].as_str().unwrap().to_string()),
            );
            evt.add_band(raw["performer"]["name"].to_string());
            result.push(evt);
        }

        result
    }
}
