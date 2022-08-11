use std::str;
use std::str::FromStr;

use reqwest::blocking::Client;
use rss::Channel;
use serde_json::Value;

pub(crate) mod date;
pub(crate) mod image;

pub struct HTTP {
    client: Client,
}

impl HTTP {
    pub(crate) fn new() -> HTTP {
        HTTP {
            client: Client::new(),
        }
    }

    pub fn get_raw(&self, url: &str) -> Vec<u8> {
        println!("get: {:?}", url);

        let mut buf: Vec<u8> = vec![];
        self.client.get(url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:96.0) Gecko/20100101 Firefox/96.0")
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")
            .header("Accept-Language", "de,en-US;q=0.7,en;q=0.3")
            .header("Accept-Encoding", "identity")
            .header("DNT", "1")
            .header("Pragma", "no-cache")
            .header("Cache-Control", "no-cache")
            .send().unwrap().copy_to(&mut buf).unwrap();
        buf
    }

    pub fn get(&self, url: &str) -> String {
        String::from_utf8(self.get_raw(url)).unwrap()
    }

    pub fn get_json(&self, url: &str) -> Value {
        let resp = self.get(url);
        serde_json::from_str(resp.as_str()).unwrap()
    }

    pub fn get_rss(&self, url: &str) -> Channel {
        let resp = self.get(url);
        Channel::from_str(resp.as_str()).unwrap()
    }
}
