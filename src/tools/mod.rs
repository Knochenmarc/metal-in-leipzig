use std::collections::HashMap;
use std::str;
use std::thread::sleep;
use std::time::Duration;

use reqwest::blocking::{Client, ClientBuilder};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Error;
use serde_json::Value;

pub(crate) mod date;
pub(crate) mod image;

pub struct Http {
    client: Client,
}

impl Http {
    pub(crate) fn new(accepts_invalid_certs: bool) -> Http {
        let mut headers = HeaderMap::new();
        headers.insert(
            "User-Agent",
            HeaderValue::from_static(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/109.0",
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
            .pool_max_idle_per_host(0) // https://github.com/hyperium/hyper/issues/2136#issuecomment-861826148
            .danger_accept_invalid_certs(accepts_invalid_certs);

        Http {
            client: builder.build().unwrap(),
        }
    }

    pub fn get_raw(&self, url: &str) -> Result<Vec<u8>, Error> {
        println!("get: {:?}", url);

        match self.client.get(url).send() {
            Ok(response) => match response.error_for_status() {
                Ok(mut response) => {
                    let mut buf: Vec<u8> = vec![];
                    response.copy_to(&mut buf).unwrap();
                    Ok(buf)
                }
                Err(error) => {
                    if error.is_status() && error.status().unwrap().as_u16() == 520 {
                        sleep(Duration::from_secs(5));
                        self.get_raw(url)
                    } else {
                        Err(error)
                    }
                }
            },
            Err(error) => {
                if (error.is_status() && error.status().unwrap().as_u16() == 520)
                    || error.is_timeout()
                {
                    sleep(Duration::from_secs(5));
                    self.get_raw(url)
                } else {
                    Err(error)
                }
            }
        }
    }

    pub fn get(&self, url: &str) -> Result<String, Error> {
        self.get_raw(url)
            .map(|data| String::from_utf8(data).unwrap())
    }

    pub fn get_json(&self, url: &str) -> Result<Value, Error> {
        self.get(url)
            .map(|data| serde_json::from_str(data.as_str()).unwrap())
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
