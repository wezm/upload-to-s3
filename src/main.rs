use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::process::ExitCode;
use std::{env, fs};

use getopts::Options;
use ureq::AgentBuilder;
use url::Url;

type HeaderMap = HashMap<String, String>;

struct RequestData {
    access: String,
    secret: String,
    bucket: String,
    key: String,
    region: String,
}

fn main() -> ExitCode {
    dotenv::dotenv().ok();

    match try_main() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Upload failed: {}", err);
            ExitCode::FAILURE
        }
    }
}

fn try_main() -> Result<(), Box<dyn Error>> {
    let args = parse_args()?;
    let aws_access_key_id = env::var("AWS_ACCESS_KEY_ID")?;
    let aws_secret_access_key = env::var("AWS_SECRET_ACCESS_KEY")?;
    let headers = HeaderMap::new();
    let len = fs::metadata(&args.local_filename)?.len();
    let file = File::open(&args.local_filename)?;
    let rd = RequestData {
        access: aws_access_key_id,
        secret: aws_secret_access_key,
        bucket: args.bucket,
        key: args.object_name.clone(),
        region: args.region,
    };

    println!("Uploading...");
    upload_object(file, len, &rd, &headers)?;
    println!("Uploaded {}", args.object_name);
    Ok(())
}

struct Args {
    bucket: String,
    region: String,
    local_filename: PathBuf,
    object_name: String,
}

fn parse_args() -> Result<Args, getopts::Fail> {
    let args: Vec<_> = env::args().collect();
    let program = &args[0];

    let mut opts = Options::new();
    opts.optopt("b", "bucket", "S3 bucket", "NAME");
    opts.optopt("r", "region", "S3 region (default: us-east-1)", "REGION");
    opts.optflag("h", "help", "Print this help information");
    let matches = opts.parse(&args[1..])?;

    let bucket = matches
        .opt_str("b")
        .unwrap_or_else(|| usage(program, &opts));
    let region = matches
        .opt_str("r")
        .unwrap_or_else(|| "us-east-1".to_string());

    let local_filename = matches.free.get(0).unwrap_or_else(|| usage(program, &opts));
    let object_name = matches.free.get(1).unwrap_or_else(|| usage(program, &opts));
    Ok(Args {
        bucket,
        region,
        local_filename: PathBuf::from(local_filename),
        object_name: object_name.to_string(),
    })
}

fn upload_object(
    data: impl std::io::Read,
    len: u64,
    req_data: &RequestData,
    headers: &HeaderMap,
) -> Result<(), Box<dyn Error>> {
    // Not using virtual host based endpoint because it doesn't work with buckets with dots
    // in their name. This post from 2020 about the deprecation of path based endpoints claims
    // they are working on a solution to this but as of 2024 there doesn't seem to be one.
    // https://aws.amazon.com/blogs/aws/amazon-s3-path-deprecation-plan-the-rest-of-the-story/
    // let url: Url = format!(
    //     "https://{}.s3.{}.amazonaws.com/",
    //     req_data.bucket, req_data.region
    // )
    // .parse()?;
    let url: Url = format!(
        "https://s3.{}.amazonaws.com/{}/{}",
        req_data.region, req_data.bucket, req_data.key
    )
    .parse()?;
    let signature = s3v4::signature(
        &url,
        "PUT",
        &req_data.access,
        &req_data.secret,
        &req_data.region,
        &"s3",
        "UNSIGNED-PAYLOAD",
    )?;

    let agent = AgentBuilder::new().build();
    let mut req = agent
        .put(url.as_str())
        .set("x-amz-content-sha256", "UNSIGNED-PAYLOAD")
        .set("x-amz-date", &signature.date_time)
        .set("authorization", &signature.auth_header)
        .set("content-length", &len.to_string());
    for (k, v) in headers {
        req = req.set(k, v);
    }

    let response = req.send(data)?;
    if response.status() >= 300 {
        let status = response.status();
        let body = response.into_string()?;
        return Err(format!("Error - {}\n{}", status, body).into());
    }
    Ok(())
}

fn usage(program: &str, opts: &Options) -> ! {
    let brief = format!("Usage: {} [options] FILE OBJECT", program);
    eprint!("{}", opts.usage(&brief));
    std::process::exit(1);
}
