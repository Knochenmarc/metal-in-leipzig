use std::collections::HashMap;
use std::str;
use std::str::FromStr;

use reqwest::blocking::{Client, ClientBuilder};
use reqwest::header::{HeaderMap, HeaderValue};
use rss::Channel;
use serde_json::Value;

pub(crate) mod date;
pub(crate) mod image;

pub struct HTTP {
    client: Client,
}

impl HTTP {
    pub(crate) fn new(accepts_invalid_certs: bool) -> HTTP {
        let mut headers = HeaderMap::new();
        headers.insert(
            "User-Agent",
            HeaderValue::from_static(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:96.0) Gecko/20100101 Firefox/96.0",
            ),
        );
        headers.insert("Accept", HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"));
        headers.insert(
            "Accept-Language",
            HeaderValue::from_static("de,en-US;q=0.7,en;q=0.3"),
        );
        headers.insert("Accept-Encoding", HeaderValue::from_static("identity"));
        headers.insert("DNT", HeaderValue::from_static("1"));
        headers.insert("Pragma", HeaderValue::from_static("no-cache"));
        headers.insert("Cache-Control", HeaderValue::from_static("no-cache"));

        let builder = ClientBuilder::new()
            .default_headers(headers)
            .danger_accept_invalid_certs(accepts_invalid_certs);

        HTTP {
            client: builder.build().unwrap(),
        }
    }

    pub fn get_raw(&self, url: &str) -> Vec<u8> {
        println!("get: {:?}", url);

        let mut buf: Vec<u8> = vec![];
        self.client
            .get(url)
            .send()
            .unwrap()
            .copy_to(&mut buf)
            .unwrap();
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

    pub fn post(&self, url: &str, payload: HashMap<&str, &str>) -> String {
        println!("post: {:?}", url);

        let mut buf: Vec<u8> = vec![];

        self.client
            .post(url)
            .form(&payload)
            .send()
            .unwrap()
            .copy_to(&mut buf)
            .unwrap();

        String::from_utf8(buf).unwrap()
    }

    pub fn post_json(&self, url: &str, payload: HashMap<&str, &str>) -> Value {
        let resp = self.post(url, payload);
        serde_json::from_str(resp.as_str()).unwrap()
    }
}
