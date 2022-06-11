use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok(); // Grabbing ENV vars

    // Pull DATABASE_URL env var
    let database_url = env::var("MONGODB_URL").expect("MONGODB_URL must be set");

    println!("cargo:rustc-env=MONGODB_URL={database_url}");
}
