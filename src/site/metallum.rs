use lazy_static::lazy_static;
use regex::Regex;

use crate::event::BandInfo;
use crate::Http;

pub fn find_band(band: &mut BandInfo, http: &Http) {
    lazy_static! {
        static ref REG: Regex = Regex::new("(?i)<a href=\"(.*?)\">").unwrap();
    }

    let name = &band.name;

    let json = http.get_json(format!("{}{}", r"https://www.metal-archives.com/search/ajax-advanced/searching/bands/?status=1&exactBandMatch=1&bandName=", name).as_str()).unwrap();
    if json["iTotalRecords"].as_i64().unwrap_or(0) == 1 {
        let first = json["aaData"][0].as_array().unwrap();
        let url = REG.captures(first[0].as_str().unwrap()).unwrap();
        band.genre = Option::Some(first[1].to_string());
        band.metallum_link = Option::Some(url[1].to_string());
    }
}
