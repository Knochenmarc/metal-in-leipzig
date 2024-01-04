use crate::event::{Event, Location};
use crate::site::facebook::fetch_facebook_events;
use crate::site::{metallum, spirit_of_metal, Filter, HasMetalBands, Site};
use crate::tools::Http;

pub struct Muehlkeller<'l> {
    location: Location<'l, 'l, 'l>,
}

impl Muehlkeller<'_> {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "mk",
                name: "Mühlkeller",
                website: "https://www.muehlstrasse.de/",
            },
        }
    }
}

impl Site for Muehlkeller<'_> {
    fn get_location(&self) -> &Location {
        &self.location
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut result = Vec::new();

        let has_metal_band = HasMetalBands {};
        let fb_events = fetch_facebook_events(http, self.get_location(), "MuehlkellerLeipzig");

        for fb_event in fb_events.iter() {
            let mut event = fb_event.clone();
            let name = event
                .name
                .replace("live im Mühlkeller", "")
                .replace("im Mühlkeller", "");
            let chunks: Vec<&str> = name.split(['&', ',']).collect();
            for chunk in chunks {
                event.add_band(chunk.trim().to_string());
            }

            for band in event.bands.iter_mut() {
                spirit_of_metal::find_band(band, http);
                metallum::find_band(band, http);
            }

            if has_metal_band.is_it_metal(&event) {
                event.name = name;
                result.push(event);
            }
        }

        result
    }
}
