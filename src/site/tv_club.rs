use crate::event::{Event, Location};
use crate::site::{metallum, spirit_of_metal, Filter, HasMetalBands, Site};
use crate::tools::date::parse_iso_date;
use crate::tools::Http;
use chrono::Months;
use html_escape::decode_html_entities;
use regex::Regex;
use reqwest::header;
use reqwest::header::HeaderMap;
use serde_json::Value;
use std::borrow::Borrow;
use std::collections::HashMap;

pub(crate) struct TVClub<'l> {
    location: Location<'l, 'l, 'l>,
}

impl TVClub<'_> {
    pub fn new() -> Self {
        Self {
            location: Location {
                slug: "tv",
                name: "TV-Club Leipzig",
                website: "https://www.tv-club-leipzig.de/",
            },
        }
    }
}

impl Site for TVClub<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let html = http.get("https://www.tv-club-leipzig.de/events/").unwrap();

        let api_settings_reg = Regex::new(r"(?i)var wpApiSettings = (\{.+});").unwrap();
        let api_settings = api_settings_reg
            .captures(&html)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();
        let api_settings: Value = serde_json::from_str(api_settings).unwrap();

        let mut request_headers = HeaderMap::new();
        request_headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());

        let mut payload = HashMap::new();
        payload.insert(
            "_wpnonce",
            api_settings.get("nonce").unwrap().as_str().unwrap(),
        );
        payload.insert("queryAttr[postType]", "post");
        payload.insert("queryAttr[queryPreset]", "");
        payload.insert("queryAttr[taxonomyRelation]", "AND");
        payload.insert("queryAttr[selectedCategories][]", "25"); // events?
        payload.insert("queryAttr[postsPerPage]", "-1");
        payload.insert("queryAttr[postsOrderBy]", "date");
        payload.insert("queryAttr[postsOrder]", "desc");
        payload.insert("queryAttr[postsSearch]", "");
        payload.insert("queryAttr[postsOffset]", "0");
        payload.insert("queryAttr[isExcludeCurrent]", "false");
        payload.insert("queryAttr[isExcludeSticky]", "false");
        payload.insert("queryAttr[currentPostId]", "233");
        payload.insert("queryAttr[fImgSize]", "full");
        payload.insert("queryAttr[metaDateFormat]", "Y-m-d");
        payload.insert("queryAttr[isExcerptFromContent]", "false");
        payload.insert("queryAttr[excerptLength]", "255");
        payload.insert("pageNumber", "1");
        payload.insert("action", "apbPosts");

        let json = http.post_json(
            "https://www.tv-club-leipzig.de/wp-admin/admin-ajax.php",
            payload,
            request_headers,
        );

        let title_reg = Regex::new(r"(?i) am \d\d\.\d\d\.\d\d\d\d").unwrap();
        let today = chrono::Utc::now().date_naive();

        let mut result: Vec<Event> = vec![];
        let has_metal_band = HasMetalBands {};

        for v in json
            .get("data")
            .unwrap()
            .as_object()
            .unwrap()
            .get("posts")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
        {
            let mut title = v.get("title").unwrap().as_str().unwrap().to_string();
            let lowered_title = title.to_lowercase();
            if lowered_title.contains("party")
                || lowered_title.contains("semester")
                || lowered_title.contains("fasching")
                || lowered_title.contains("markt")
                || title.contains("Stand-Up Comedy")
            {
                continue;
            }
            title = title.replace(" Live ", " ");
            let title = title_reg.replace_all(&title, "").to_string();

            let date = v.get("dateGMT").unwrap().as_str().unwrap().to_string();
            let date = date.get(0..10).unwrap().to_string();
            let date = parse_iso_date(date.as_str());
            // all dates are somehow of the previous year
            let date = date.checked_add_months(Months::new(12)).unwrap();
            if today.gt(date.date().borrow()) {
                continue;
            }

            let img = v
                .get("thumbnail")
                .unwrap()
                .as_object()
                .unwrap()
                .get("url")
                .unwrap();
            let img = (img.as_str()).map(|str| str.to_string());

            let mut excerpt = v.get("excerpt").unwrap().as_str().unwrap().to_string();
            if excerpt.is_empty() {
                let page_data = http
                    .get_json(
                        format!(
                            "https://www.tv-club-leipzig.de/wp-json/wp/v2/posts/{}",
                            v.get("id").unwrap()
                        )
                        .as_str(),
                    )
                    .unwrap();
                excerpt = page_data
                    .get("excerpt")
                    .unwrap()
                    .as_object()
                    .unwrap()
                    .get("rendered")
                    .unwrap()
                    .to_string();
            }
            let lowered_excerpt = excerpt.to_lowercase();

            let mut event = Event::new(
                title.clone(),
                date,
                self.location.borrow(),
                v.get("link").unwrap().as_str().unwrap().to_string(),
                img,
            );

            if event.name.contains("Heavy Crisis Festival") || lowered_excerpt.contains("metal") {
                result.push(event);
            } else {
                event.add_band(title);
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
