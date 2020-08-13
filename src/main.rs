#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use s3::bucket::Bucket;
use s3::region::Region;
use s3::S3Error;
use s3::creds::Credentials;
use std::str;

const MESSAGE: &str = "I want to go to S3";

use std::env;

struct Information {
    name: String,
    region: Region,
    credentials: Credentials,
    bucket: String,
    location_supported: bool,
}

fn main() {
    let aws = Information {
        name: "aws".into(),
        region: "us-east-2".parse().unwrap(),
        credentials: Credentials::from_env_specific(Some("AWS_ACCESS_KEY_ID"), Some("AWS_SECRET_ACCESS_KEY"), None, None,).unwrap(),
        bucket: "rusty-kebab-bucket".to_string(),
        location_supported: true,
    };

    // Create Bucket in REGION for BUCKET
    let bucket = Bucket::new(&aws.bucket, aws.region, aws.credentials).unwrap();

    rocket::ignite()
        .mount("/hello", routes![hello])
        .launch();
}

// async fn list_files(bucket: Bucket) -> Result<impl warp::Reply, warp::Rejection> {
//     let mut result = Vec::new();

//     // List out contents of directory
//     let results = bucket.list_blocking("".to_string(), None)?;

//     for (list, code) in results {
//         if (code == 200) {
//             result.push(list.name);
//         }
//     }

//     Ok(warp::reply::json(
//         &result
//     ))
// }

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn test() -> Result<(), S3Error> {
    let aws = Information {
        name: "aws".into(),
        region: "us-east-2".parse()?,
        credentials: Credentials::from_env_specific(Some("AWS_ACCESS_KEY_ID"), Some("AWS_SECRET_ACCESS_KEY"), None, None,)?,
        bucket: "rusty-kebab-bucket".to_string(),
        location_supported: true,
    };

    // Create Bucket in REGION for BUCKET
    let bucket = Bucket::new(&aws.bucket, aws.region, aws.credentials)?;

    // List out contents of directory
    let results = bucket.list_blocking("".to_string(), None)?;
    for (list, code) in results {
        assert_eq!(200, code);
        println!("{:?}", list.name);
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
