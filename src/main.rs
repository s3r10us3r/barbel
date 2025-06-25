use barbel::run;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("barbel {VERSION} by s3r10us3r");
    run();
}
