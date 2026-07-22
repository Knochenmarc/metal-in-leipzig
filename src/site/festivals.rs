use std::borrow::Borrow;

use crate::event::{Event, EventType, Location};
use crate::site::{metallum, spirit_of_metal, Filter, HasMetalBands, Site};
use crate::tools::Http;
use chrono::{Days, NaiveDate, NaiveDateTime, NaiveTime};
use regex::Regex;

pub struct Festivals<'l> {
    location: Location<'l, 'l, 'l>,
}

impl Festivals<'_> {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "fest",
                name: "Festivals",
                website: "",
            },
        }
    }

    fn fetch_dark_affair(&self, http: &Http) -> Vec<Event> {
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
                    format!("DARK AFFAIR: {}", artist),
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

impl Site for Festivals<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut festivals = vec![];

        // festivals.extend(self.fetch_dark_affair(http));

        let mut inflammen = Event::new(
            "In Flammen Open Air".to_string(),
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2026, 7, 9).unwrap(),
                NaiveTime::default(),
            ),
            self.location.borrow(),
            "https://www.in-flammen.com/".to_string(),
            // Some("https://image.jimcdn.com/app/cms/image/transf/none/path/sfa7e4f2e650d1c8b/image/i2ae80efdbf3f5e44/version/1613414208/image.jpg".to_string()),
            Some("https://image.jimcdn.com/app/cms/image/transf/none/path/sfa7e4f2e650d1c8b/image/ia34d6e16771779ee/version/1778095185/image.jpg".to_string()),
        );
        inflammen.end_date = Some(NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2026, 7, 11).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 00).unwrap(),
        ));
        festivals.push(inflammen);

        let mut impericon = Event::new(
            "Impericon Festival".to_string(),
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2026, 6, 26).unwrap(),
                NaiveTime::default(),
            ),
            self.location.borrow(),
            "https://www.impericon.com/de/festival".to_string(),
            Some("https://www.impericon.com/cdn/shop/files/20260407_imp_fest_2026_vo8_microsite_fullsize.jpg".to_string()),
        );
        impericon.end_date = Some(NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2026, 6, 27).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 00).unwrap(),
        ));
        festivals.push(impericon);

        let mut full_rewind = Event::new(
            "Full Rewind".to_string(),
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2026, 7, 30).unwrap(),
                NaiveTime::default(),
            ),
            self.location.borrow(),
            "https://full-rewind.de/".to_string(),
            Some("https://cdn.shopify.com/s/files/1/0778/0528/9815/files/260330-FRF2026-Bands_Ankundigung.jpg".to_string()),
        );
        full_rewind.end_date = Some(NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2026, 8, 1).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 00).unwrap(),
        ));
        festivals.push(full_rewind);

        let mut nexus = Event::new(
            "NEXUS - Nerd Rock Festival".to_string(),
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2026, 9, 18).unwrap(),
                NaiveTime::default(),
            ),
            self.location.borrow(),
            "https://www.nexo-nerd-expo.com/".to_string(),
            Some(
                "https://ugc.production.linktr.ee/f2acc9af-3a53-434c-afe9-1716ad1440ce_image.png"
                    .to_string(),
            ),
        );
        nexus.end_date = Some(NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2026, 9, 20).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 00).unwrap(),
        ));
        festivals.push(nexus);

        let mut metalpest = Event::new(
            "MetalPest".to_string(),
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2026, 10, 23).unwrap(),
                NaiveTime::default(),
            ),
            self.location.borrow(),
            "https://metalpest.de//".to_string(),
            Some(
                "https://metalpest.de/wp-content/uploads/2025/05/metalpest-logo-white-1.png"
                    .to_string(),
            ),
        );
        metalpest.end_date = Some(NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2026, 10, 24).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 00).unwrap(),
        ));
        festivals.push(metalpest);

        festivals
            .iter_mut()
            .for_each(|festival| festival.evt_type = EventType::Festival);

        festivals
    }
}
