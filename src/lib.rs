use rust_warc::{WarcError, WarcReader, WarcRecord};
pub struct WarcResult(pub WarcRecord);
impl WarcResult {
    pub fn unwrap(&self) -> &WarcRecord {
        match &self {
            WarcResult(WarcRecord) => return WarcRecord
        }
    }
}
#[derive(Debug, Clone)]
pub struct Buf {
    pub buffers: Vec<u8>,
    pub sum: usize,
}
impl Buf {
    pub fn new() -> Buf {
        Buf {
            buffers: [].to_vec(),
            sum: 0,
        }
    }
    pub fn add_to_buf(&mut self, chunk: &[u8]) {
        let v = chunk.to_vec();
        self.buffers = [&self.buffers[..], &v[..]].concat();
    }
    pub fn attempt_drain(&mut self, f: &dyn Fn(&WarcResult) -> bool) {
        let buf = &self.buffers[..];
        let mut warc = WarcReader::new(buf);
        loop {
            let item: Option<Result<WarcRecord, WarcError>> = warc.next();
            match item {
                Some(Ok(record)) => {
                    self.sum = self.sum + 1;
                    f(&WarcResult(record));
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

pub async fn process_warc(
    url: String,
    f: &dyn Fn(&WarcResult) -> bool,
) -> Result<Vec<bool>, Box<dyn std::error::Error>> {
    let mut buf = Buf::new();
    let mut res = reqwest::get(&url).await?;
    while let Some(chunk) = res.chunk().await? {
        buf.add_to_buf(&chunk[..]);
        buf.attempt_drain(f);
    }
    std::result::Result::Ok([true].to_vec())
}