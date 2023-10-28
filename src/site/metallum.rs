use lazy_static::lazy_static;
use regex::Regex;

use crate::event::BandInfo;
use crate::Http;

pub fn find_band(band: &mut BandInfo, http: &Http) {
    lazy_static! {
        static ref REG: Regex = Regex::new("(?i)<a href=\"(.*?)\">").unwrap();
    }

    let name = &band.name;

    let url = format!(
        "{}{}",
        r"https://www.metal-archives.com/search/ajax-advanced/searching/bands/?status=1&exactBandMatch=1&bandName=",
        name
    );
    let response = http.get_json(url.as_str());
    if response.is_err() {
        return;
    }

    let json = response.unwrap();
    if json["iTotalRecords"].as_i64().unwrap_or(0) == 1 {
        let first = json["aaData"][0].as_array().unwrap();
        let url = REG.captures(first[0].as_str().unwrap()).unwrap();
        band.genre = Some(first[1].to_string());
        band.metallum_link = Some(url[1].to_string());
    }
}
