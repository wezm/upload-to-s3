use std::env;
use std::error::Error;
use std::process::ExitCode;

use getopts::Options;
use rusoto_core::{HttpClient, Region};
use rusoto_credential::EnvironmentProvider;
use rusoto_s3::{PutObjectRequest, S3Client, StreamingBody, S3};
use tokio_util::io::ReaderStream;

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    dotenv::dotenv().ok();

    match try_main().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Upload failed: {}", err);
            ExitCode::FAILURE
        }
    }
}

async fn try_main() -> Result<(), Box<dyn Error>> {
    let (bucket, local_filename, object_name) = parse_args()?;

    let client = S3Client::new_with(
        HttpClient::new()?,
        EnvironmentProvider::default(),
        Region::UsEast1,
    );

    let meta = ::std::fs::metadata(&local_filename)?;
    let file = tokio::fs::File::open(&local_filename).await?;
    let read_stream = ReaderStream::new(file);
    let req = PutObjectRequest {
        bucket: bucket.to_owned(),
        key: object_name.to_owned(),
        content_length: Some(meta.len() as i64),
        body: Some(StreamingBody::new(read_stream)),
        ..Default::default()
    };

    println!("Uploading...");
    client.put_object(req).await?;
    println!("Uploaded {}", object_name);
    Ok(())
}

fn parse_args() -> Result<(String, String, String), getopts::Fail> {
    let args: Vec<_> = env::args().collect();
    let program = &args[0];

    let mut opts = Options::new();
    opts.optopt("b", "bucket", "S3 bucket", "NAME");
    opts.optflag("h", "help", "Print this help information");
    let matches = opts.parse(&args[1..])?;

    let bucket = matches
        .opt_str("b")
        .unwrap_or_else(|| usage(program, &opts));

    let local_filename = matches.free.get(0).unwrap_or_else(|| usage(program, &opts));
    let object_name = matches.free.get(1).unwrap_or_else(|| usage(program, &opts));
    Ok((bucket, local_filename.to_string(), object_name.to_string()))
}

fn usage(program: &str, opts: &Options) -> ! {
    let brief = format!("Usage: {} [options] FILE OBJECT", program);
    eprint!("{}", opts.usage(&brief));
    std::process::exit(1);
}
