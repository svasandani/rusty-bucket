use s3::bucket::Bucket;
use s3::region::Region;
use s3::S3Error;
use s3::creds::Credentials;
use std::str;

use warp::Filter;

const MESSAGE: &str = "I want to go to S3";

use std::env;

struct Storage {
    name: String,
    region: Region,
    credentials: Credentials,
    bucket: String,
    location_supported: bool,
}

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn test() -> Result<(), S3Error> {
    // let bucket_name = &env::var("S3_BUCKET_NAME").unwrap();
    // let region = &env::var("AWS_REGION").unwrap();
    let aws = Storage {
        name: "aws".into(),
        region: "us-east-2".parse()?,
        credentials: Credentials::from_env_specific(Some("AWS_ACCESS_KEY_ID"), Some("AWS_SECRET_ACCESS_KEY"), None, None,)?,
        bucket: "rusty-kebab-bucket".to_string(),
        location_supported: true,
    };

    // let bucket = Bucket::new(bucket_name, region, credentials);

    println!("Running {}", aws.name);
    // Create Bucket in REGION for BUCKET
    let bucket = Bucket::new(&aws.bucket, aws.region, aws.credentials)?;

    // List out contents of directory
    let results = bucket.list_blocking("".to_string(), None)?;
    for (list, code) in results {
        assert_eq!(200, code);
        println!("{:?}", list.contents);
    }

    let (_, code) =
        bucket.put_object_blocking("test_file", MESSAGE.as_bytes(), "text/plain")?;
    // println!("{}", bucket.presign_get("test_file", 604801)?);
    assert_eq!(200, code);

    // Get the "test_file" contents and make sure that the returned message
    // matches what we sent.
    let (data, code) = bucket.get_object_blocking("test_file")?;
    let string = str::from_utf8(&data)?;
    println!("{}", string);
    assert_eq!(200, code);
    assert_eq!(MESSAGE, string);

    if aws.location_supported {
        // Get bucket location
        println!("{:?}", bucket.location_blocking()?);
    }

    let (_, code) = bucket.delete_object_blocking("test_file")?;

    Ok(())
}
