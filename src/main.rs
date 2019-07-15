use std::default::Default;
use std::env;

use futures::stream::Stream;
use futures_fs::FsPool;
use getopts::Options;

use rusoto_core::{HttpClient, Region};
use rusoto_credential::EnvironmentProvider;
use rusoto_s3::{PutObjectRequest, S3Client, StreamingBody, S3};

fn main() {
    dotenv::dotenv().ok();

    let args: Vec<_> = env::args().collect();
    let program = &args[0];

    let mut opts = Options::new();
    opts.optopt("b", "bucket", "S3 bucket", "NAME");
    opts.optflag("h", "help", "Print this help information");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    let bucket = matches
        .opt_str("b")
        .unwrap_or_else(|| usage(program, &opts));
    let local_filename = matches.free.get(0).unwrap_or_else(|| usage(program, &opts));
    let object_name = matches.free.get(1).unwrap_or_else(|| usage(program, &opts));

    let client = S3Client::new_with(
        HttpClient::new().expect("failed to create request dispatcher"),
        EnvironmentProvider::default(),
        Region::UsEast1,
    );

    let meta = ::std::fs::metadata(local_filename).unwrap();
    let fs = FsPool::default();
    let read_stream = fs.read(local_filename.to_owned(), Default::default());
    let req = PutObjectRequest {
        bucket: bucket.to_owned(),
        key: object_name.to_owned(),
        content_length: Some(meta.len() as i64),
        body: Some(StreamingBody::new(read_stream)),
        ..Default::default()
    };

    println!("Uploading...");
    client.put_object(req).sync().expect("Couldn't PUT object");
}

fn usage(program: &str, opts: &Options) -> ! {
    let brief = format!("Usage: {} [options] FILE OBJECT", program);
    eprint!("{}", opts.usage(&brief));
    std::process::exit(1);
}
