use s3::bucket::Bucket;
use s3::region::Region;
use awscreds::Credentials;
use std::env;

fn main() {
    let bucket_name = &env::var("S3_BUCKET_NAME").unwrap();
    let region = &env::var("AWS_REGION").unwrap();
    let credentials = Credentials::new(None, None, None, None, None);

    let bucket = Bucket::new(bucket_name, region, credentials);
}
