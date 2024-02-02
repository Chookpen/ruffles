use std::env;
use std::fs::File;
use std::io::Write;
use std::process::{Command, exit};

async fn download_file(web_address: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(web_address).await?;

    if response.status().is_success() {
        let mut file = File::create(file_path)?;
        file.write_all(&response.bytes().await?).expect("Failed to write file");
        Ok(())
    } else {
        eprintln!("Failed to download file. Status code: {}", response.status());
        exit(1);
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        exit(1);
    }

    let filename = &args[1];
    let web_address = "https://raw.githubusercontent.com/Chookpen/ruffles-scripts/main/scripts/";
    let web_file_path = format!("{}{}", web_address, filename);

    if let Err(err) = download_file(&web_file_path, filename).await {
        eprintln!("Failed to make a request: {}", err);
        exit(1);
    }

    Command::new("chmod")
        .arg("+x")
        .arg(filename)
        .status()
        .expect("Failed to run chmod");

    Command::new(format!("./{}", filename))
        .status()
        .expect("Failed to run the downloaded file");
}

