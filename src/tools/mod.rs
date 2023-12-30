use reqwest::blocking::Client;
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

    pub fn get(&self, url: &str) -> String {
        println!("get: {:?}", url);

        return self.client.get(url)
            .header("User-Agent","Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:96.0) Gecko/20100101 Firefox/96.0")
            .header("Accept","text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")
            .header("Accept-Language", "de,en-US;q=0.7,en;q=0.3")
            .header("Accept-Encoding", "identity")
            .header("DNT", "1")
            .header("Pragma", "no-cache")
            .header("Cache-Control", "no-cache")
            .send().unwrap().text().unwrap();
    }

    pub fn get_json(&self, url: &str) -> Value {
        let resp = self.get(url);
        return serde_json::from_str(resp.as_str()).unwrap();
    }
}
