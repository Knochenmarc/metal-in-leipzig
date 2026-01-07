use crate::event::{Event, Location};
use crate::site::{
    metallum, parse_linked_data_events, spirit_of_metal, Filter, HasMetalBands, Site,
};
use crate::tools::date::parse_iso_datetime;
use crate::tools::Http;
use html_escape::decode_html_entities;
use std::borrow::Borrow;

pub struct GeyserHaus<'l> {
    location: Location<'l, 'l, 'l>,
}

impl GeyserHaus<'_> {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "gh",
                name: "GeyserHaus",
                website: "https://www.geyserhaus.de/veranstaltungen/",
            },
        }
    }
}

impl Site for GeyserHaus<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut result = Vec::new();
        let has_metal_band = HasMetalBands {};

        let html = http
            .get("https://www.geyserhaus.de/veranstaltungen/")
            .unwrap();
        for data_event in parse_linked_data_events(html.as_str()) {
            let name = decode_html_entities(data_event["name"].as_str().unwrap()).to_string();
            let image = data_event["image"].as_str().unwrap();
            let start_date = data_event["startDate"].as_str().unwrap();
            let end_date = data_event["endDate"].as_str().unwrap();
            let url = data_event["url"].as_str().unwrap();

            let mut event = Event::new(
                name.clone(),
                parse_iso_datetime(start_date).unwrap(),
                self.location.borrow(),
                url.to_string(),
                Some(image.to_string()),
            );
            event.end_date = Some(parse_iso_datetime(end_date).unwrap());

            event.add_band(name);

            for band in event.bands.iter_mut() {
                spirit_of_metal::find_band(band, http);
                metallum::find_band(band, http);
            }

            if has_metal_band.is_it_metal(event.borrow()) {
                result.push(event);
            }
        }

        result
    }
}
