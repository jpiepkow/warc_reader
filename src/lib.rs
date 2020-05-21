use rust_warc::{WarcError, WarcReader, WarcRecord};
use std::collections::HashMap;

pub enum Warc {
    Wait,
    WarcResult(WarcRecord),
}
pub struct WarcParser {
    pub records: Vec<WarcRecord>,
    pub response: reqwest::Response,
    pub buffers: Vec<u8>,
    pub can_poll: bool,
    pub record_ids: HashMap<String, bool>,
}

impl WarcParser {
    pub async fn new(url: String) -> Result<WarcParser, Box<dyn std::error::Error>> {
        Ok(WarcParser {
            buffers: [].to_vec(),
            records: vec![],
            response: reqwest::get(&url).await?,
            can_poll: true,
            record_ids: HashMap::new(),
        })
    }
    pub async fn next(&mut self) -> Option<Warc> {
        if self.can_poll {
            if let Some(chunk) = self.response.chunk().await.unwrap() {
                self.attempt_drain(&chunk[..]);
            } else {
                self.can_poll = false;
            }
        }
        if self.records.len() == 0 && self.can_poll == false {
            return None;
        } else {
            if self.records.len() == 0 {
                return Some(Warc::Wait);
            } else {
                return Some(Warc::WarcResult(self.records.remove(0)));
            }
        }
    }
    pub fn attempt_drain(&mut self, chunk: &[u8]) {
        self.buffers = [&self.buffers[..], &chunk[..]].concat();
        let mut warc = WarcReader::new(&self.buffers[..]);
        loop {
            let item: Option<Result<WarcRecord, WarcError>> = warc.next();
            match item {
                Some(Ok(record)) => {
                    if let Some(id) = record.header.get(&"warc-record-id".into()) {
                        let new_id = id.clone();
                        if !self.record_ids.contains_key(id) {
                            if record.header.get(&"warc-target-uri".into()).is_some() {
                                self.records.push(record);
                                self.record_ids.insert(new_id.to_string(), true);
                            }
                        }
                    }
                }
                Some(Err(_e)) => {
                    self.buffers = self.buffers[warc.sum..].to_vec();
                    break;
                }
                None => {
                    break;
                }
            }
        }
    }
}
