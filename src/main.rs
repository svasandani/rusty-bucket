use s3::bucket::Bucket;
use s3::region::Region;
use awscreds::Credentials;
use std::env;

struct Storage {
    name: String,
    region: Region,
    credentials: Credentials,
    bucket: String,
    location_supported: bool,
}

fn main() {
    // let bucket_name = &env::var("S3_BUCKET_NAME").unwrap();
    // let region = &env::var("AWS_REGION").unwrap();
    let aws = Storage {
        name: "aws".into(),
        region: "us-east-1".parse()?,
        credentials: Credentials::from_env_specific(Some("AWS_ACCESS_KEY_ID"), Some("AWS_SECRET_ACCESS_KEY"), None, None,)?,
        bucket: "rusty-kebab=bucket".to_string(),
        location_support: true
    };

    // let bucket = Bucket::new(bucket_name, region, credentials);

    println!("Running {}", backend.name);
    // Create Bucket in REGION for BUCKET
    let bucket = Bucket::new(&backend.bucket, backend.region, backend.credentials)?;

    // List out contents of directory
    let results = bucket.list_blocking("".to_string(), None)?;
    for (list, code) in results {
        assert_eq!(200, code);
        println!("{:?}", list.contents.len());
    }

    Ok(())
}
