Minimum repo case:

```
fn test_fnc(record: &warc_reader::WarcResult) -> bool {
	println!("{:?}", "=============");
	println!("{:?}", record.unwrap().header.get(&"Warc-Record-ID".into()).unwrap());
	println!("{:?}", "=============");
	return true
}

#[tokio::main]
async fn main() {
	warc_reader::process_warc("https://commoncrawl.s3.amazonaws.com/crawl-data/CC-MAIN-2020-05/segments/1579250589560.16/warc/CC-MAIN-20200117123339-20200117151339-00036.warc.gz".to_string(),&test_fnc).await;
}
```
