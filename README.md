Description:
Fast streaming Warc parser.

Give the parser a url to a gzipped WARC file and it will stream the individual results of the file back to you without pulling the entire file into memory.

Minimum repo case:

```
let url = "https://commoncrawl.s3.amazonaws.com/crawl-data/CC-MAIN-2020-05/segments/1579250589560.16/warc/CC-MAIN-20200117123339-20200117151339-00036.warc.gz".to_string();
    let mut warc_parser = WarcParser::new(url).await.unwrap();
    loop {
        match warc_parser.next().await {
            Some(Warc::Wait) => continue,
            Some(Warc::WarcResult(result)) => {
                // Do something with result
            }
            None => break,
        }
    }
```
