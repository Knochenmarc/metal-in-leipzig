use std::collections::HashMap;

use regex::Regex;

use crate::site::Filter;
use crate::{Event, Http};

pub struct Mawi {
    collected_dates: HashMap<String, String>,
}

impl Mawi {
    pub fn new(location: &str, http: &Http) -> Self {
        let mut event_ids = {
            let reg = Regex::new("(?i)class='grid-item framepic (?P<cat>\\d+)' data-category=''>\\s*<a href='index\\.php\\?menus_id=2&solo=1&id=(?P<id>\\d+)'").unwrap();
            let html = http
                .get("https://www.mawi-concert.de/index.php?menus_id=2")
                .unwrap();
            let mut map: HashMap<String, String> = HashMap::new();
            for capture in reg.captures_iter(html.as_str()) {
                map.insert(
                    capture.name("id").unwrap().as_str().to_string(),
                    capture.name("cat").unwrap().as_str().to_string(),
                );
            }
            map
        };

        let collected_dates = {
            let mut result = HashMap::new();
            let mut payload = HashMap::new();
            let location = location.to_owned() + "##location##";
            payload.insert("sk", location.as_str());
            let html = http.post(
                "https://www.mawi-concert.de/worker/searching_inc.php",
                payload,
            );
            let reg = Regex::new(
                "(?is)index.php\\?menus_id=2&solo=1&id=(?P<id>\\d+)&.*?am (?P<date>\\d\\d\\.\\d\\d\\.\\d\\d\\d\\d)",
            ).unwrap();
            for capture in reg.captures_iter(html.as_str()) {
                let id = capture.name("id").unwrap().as_str();
                let date = capture.name("date").unwrap().as_str();
                let cat = event_ids.remove(id).unwrap();
                result.insert(date.to_string(), cat.to_string());
            }

            result
        };

        Self { collected_dates }
    }
}

impl Filter for Mawi {
    fn is_it_metal(&self, evt: &Event) -> bool {
        let date = evt.start_date.format("%d.%m.%Y").to_string();

        match self.collected_dates.get(&*date) {
            None => false,
            Some(value) => value == "15",
        }
    }
}
