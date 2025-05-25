use std::borrow::Borrow;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::event::{Event, EventType, Location};
use crate::site::Site;
use crate::tools::Http;

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
}

impl Site for Festivals<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, _http: &Http) -> Vec<Event> {
        let mut inflammen = Event::new(
            "In Flammen Open Air".to_string(),
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2025, 7, 10).unwrap(),
                NaiveTime::default(),
            ),
            self.location.borrow(),
            "https://www.in-flammen.com/".to_string(),
            Some("https://image.jimcdn.com/app/cms/image/transf/none/path/sfa7e4f2e650d1c8b/image/if70ed78ae97ad0c5/version/1748024390/image.jpg".to_string()),
        );
        inflammen.end_date = Some(NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 7, 12).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 00).unwrap(),
        ));
        inflammen.evt_type = EventType::Festival;

        let mut rock = Event::new(
            "Rock um zu Helfen".to_string(),
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 10, 11).unwrap(),
                NaiveTime::default(),
            ),
            self.location.borrow(),
            "https://www.rock-um-zu-helfen.de/".to_string(),
            Some("https://s3-eu-west-1.amazonaws.com/static.csone.dgbrt.de/artifacts/events/466/design.png".to_string()),
        );
        rock.end_date = Some(NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 10, 12).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 00).unwrap(),
        ));
        rock.evt_type = EventType::Festival;

        let mut impericon = Event::new(
            "Impericon Festival".to_string(),
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2025, 6, 27).unwrap(),
                NaiveTime::default(),
            ),
            self.location.borrow(),
            "https://www.impericon.com/de/festival".to_string(),
            Some("https://www.impericon.com/cdn/shop/files/20250408_impfest_desktop_lineup_ticket_alert_de.jpg".to_string()),
        );
        impericon.end_date = Some(NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 6, 28).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 00).unwrap(),
        ));
        impericon.evt_type = EventType::Festival;

        let mut full_rewind = Event::new(
            "Full Rewind".to_string(),
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2025, 7, 31).unwrap(),
                NaiveTime::default(),
            ),
            self.location.borrow(),
            "https://full-rewind.de/".to_string(),
            Some("https://cdn.shopify.com/s/files/1/0778/0528/9815/files/FRF2025-Bands_Ankundigung4.jpg".to_string()),
        );
        full_rewind.end_date = Some(NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 8, 2).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 00).unwrap(),
        ));
        full_rewind.evt_type = EventType::Festival;

        let mut nexus = Event::new(
            "Nexus Festival".to_string(),
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2025, 7, 11).unwrap(),
                NaiveTime::default(),
            ),
            self.location.borrow(),
            "https://www.nexo-nerd-expo.com/".to_string(),
            Some("https://lh3.googleusercontent.com/u/0/d/11YLfyDIFN62PioQsXPzuFew2R549VRQz=w3129-h1306-iv1".to_string()),
        );
        nexus.end_date = Some(NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 7, 13).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 00).unwrap(),
        ));
        nexus.evt_type = EventType::Festival;

        vec![inflammen, rock, impericon, full_rewind, nexus]
    }
}
