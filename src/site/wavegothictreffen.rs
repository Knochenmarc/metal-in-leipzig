use chrono::Timelike;
use regex::Regex;
use std::borrow::Borrow;

use crate::event::{Event, EventType, Location};
use crate::site::{metallum, spirit_of_metal, Filter, HasMetalBands, Site};
use crate::tools::date::parse_german_date;
use crate::tools::Http;

pub struct WaveGothicTreffen<'l> {
    location: Location<'l, 'l, 'l>,
}

impl WaveGothicTreffen<'_> {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "wgt",
                name: "Wave Gothic Treffen",
                website: "https://www.wave-gotik-treffen.de/",
            },
        }
    }
}

impl Site for WaveGothicTreffen<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut result = Vec::new();

        if let Ok(html) = http.get("https://www.wave-gotik-treffen.de/prog/celebrant.php") {
            let has_metal_band = HasMetalBands {};
            let day_reg: Regex = Regex::new(r#"<h2>\w+,\s(.*)</h2>(.*)"#).unwrap();
            let row_reg: Regex =
                Regex::new(r#"<td>(\d\d)\.(\d\d)&nbsp;Uhr</td><td>([\w\s]+?) \(\w+\)</td>"#)
                    .unwrap();
            for day_group in day_reg.captures_iter(html.as_str()) {
                let day = day_group.get(1).unwrap().as_str();
                let row = day_group.get(2).unwrap().as_str();
                let date = parse_german_date(day);
                for row in row_reg.captures_iter(row) {
                    let hour = row
                        .get(1)
                        .unwrap()
                        .as_str()
                        .to_string()
                        .parse::<u32>()
                        .unwrap();
                    let minute = row
                        .get(2)
                        .unwrap()
                        .as_str()
                        .to_string()
                        .parse::<u32>()
                        .unwrap();
                    let name = row.get(3).unwrap().as_str();

                    let mut event = Event::new(
                        name.to_string(),
                        date.with_hour(hour).unwrap().with_minute(minute).unwrap(),
                        self.location.borrow(),
                        "https://www.wave-gotik-treffen.de/prog/celebrant.php".to_string(),
                        None,
                    );
                    event.evt_type = EventType::Concert;
                    event.add_band(name.to_string());
                    for band in event.bands.iter_mut() {
                        spirit_of_metal::find_band(band, http);
                        metallum::find_band(band, http);
                    }

                    if has_metal_band.is_it_metal(event.borrow()) {
                        result.push(event);
                    }
                }
            }
        }

        result
    }
}
