use rust_warc::{WarcError, WarcReader, WarcRecord};
pub enum Warc {
    Wait,
    WarcResult(WarcRecord),
}
pub struct WarcParser {
    pub records: Vec<WarcRecord>,
    pub response: reqwest::Response,
    pub buffers: Vec<u8>,
    pub can_poll: bool
}

    
impl WarcParser {
    pub async fn new(url: String) -> Result<WarcParser, Box<dyn std::error::Error>> {
        Ok(WarcParser {
            buffers: [].to_vec(),
            records: vec![],
            response: reqwest::get(&url).await?,
            can_poll: true
        })
    }
   pub async fn next(&mut self) -> Option<Warc> {
        if(self.can_poll) {
            if let Some(chunk) = self.response.chunk().await.unwrap() {
                self.add_to_buf(&chunk[..]);
                self.attempt_drain(); 
            } else {
               self.can_poll = false;
            }
        }
        if (self.records.len() == 0 && self.can_poll == false)  {
            return None;
        } else {
            if self.records.len() == 0 {
                return Some(Warc::Wait);
            } else {
                return Some(Warc::WarcResult(self.records.remove(0)));
            }
        }
   }
    pub fn add_to_buf(&mut self, chunk: &[u8]) {
        let v = chunk;
        self.buffers = [&self.buffers[..], &v[..]].concat();
    }
    pub fn attempt_drain(&mut self) {
        let buf = &self.buffers[..];
        let mut warc = WarcReader::new(buf);
        loop {
            let item: Option<Result<WarcRecord, WarcError>> = warc.next();
            match item {
                Some(Ok(record)) => {
                    self.records.push(record);
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