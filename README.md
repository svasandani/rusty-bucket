<h1 align="center">Rusty Bucket<br>
<img src="https://img.shields.io/github/languages/code-size/svasandani/rusty-bucket" />
<img src="https://img.shields.io/github/license/svasandani/rusty-bucket" />
<img src="https://img.shields.io/github/last-commit/svasandani/rusty-bucket" />
<br>
</h1>
<br>
A Rust microservice for simplifying the AWS S3 API.

Make sure to use the <strong>nightly</strong> build!

## Development
- Run the program

    `$ AWS_ACCESS_KEY_ID=... AWS_SECRET_ACCESS_KEY=... ROCKET_PORT=4117 cargo run`

## Production
- Build the program

    `$ cargo build --release --target x86_64-unknown-linux-musl`