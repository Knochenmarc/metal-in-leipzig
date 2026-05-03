use crate::event::{Event, EventType, Location};
use crate::site::{metallum, spirit_of_metal, Filter, HasMetalBands, Site};
use crate::tools::Http;
use chrono::{Days, NaiveDate, NaiveTime};
use regex::Regex;
use std::borrow::Borrow;

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

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        // return vec![];

        let start_date = NaiveDate::from_ymd_opt(2026, 5, 22).unwrap();
        let links = vec![
            "https://www.dark-affair.com/de/timetable-freitag",
            "https://www.dark-affair.com/de/timetable-samstag",
            "https://www.dark-affair.com/de/timetable-sonntag",
            "https://www.dark-affair.com/de/timetable-montag",
        ];
        let image = Some(
            "https://www.dark-affair.com/data/downloads/2026/dark-affair-banner-390x120.jpg"
                .to_string(),
        );

        let mut result = vec![];
        let has_metal_band = HasMetalBands {};

        let row_reg: Regex = Regex::new(
            r#"(?si)<span class="space time">(\d\d:\d\d) Uhr</span>.*?<span class="artist">(.*?)</span>"#,
        )
        .unwrap();

        for (day_offset, link) in links.into_iter().enumerate() {
            let date = start_date
                .checked_add_days(Days::new(day_offset as u64))
                .unwrap();
            let html = http.get(link).unwrap();

            for group in row_reg.captures_iter(html.as_str()) {
                let time = &group.get(1).unwrap().as_str();
                let artist = &group.get(2).unwrap().as_str();

                let mut event = Event::new(
                    artist.to_string(),
                    date.and_time(NaiveTime::parse_from_str(time, "%H:%M").unwrap()),
                    self.location.borrow(),
                    link.to_string(),
                    image.clone(),
                );
                event.add_band(artist.to_string());
                event.evt_type = EventType::Concert;

                for band in event.bands.iter_mut() {
                    spirit_of_metal::find_band(band, http);
                    metallum::find_band(band, http);
                }

                if has_metal_band.is_it_metal(event.borrow()) {
                    result.push(event);
                }
            }
        }

        result
    }
}
