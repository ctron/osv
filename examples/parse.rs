use std::env;
use std::fs;
use std::process;

use osv::client;

#[async_std::main]
async fn main() -> Result<(), client::ApiError> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() <= 0 {
        println!("filename expected");
        process::exit(1);
    }
    for arg in &args {
        let path: &str = arg.as_str();
        let file = fs::File::open(path).unwrap();
        let _vuln: osv::schema::Vulnerability =
            serde_json::from_reader(file).unwrap_or_else(|_| panic!("fail: {}", path));
        println!("pass: {}", path);
    }
    Ok(())
}
