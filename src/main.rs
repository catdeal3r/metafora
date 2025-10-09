use clap::Parser;

pub mod net;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Show debug information
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
    
    // File to be uploaded
    #[arg(short, long, default_value = "none")]
    file: String,

    // Url to download from
    #[arg(short, long, default_value = "none")]
    url: String,
}


fn main() {
    let cli = Cli::parse();

    if cli.file != "none" && cli.url != "none" {
        println!("You cannot download and upload a file at the same time.");
        return
    }

    if cli.file != "none" {
        let mut raw_output = String::new();
        net::upload_file_and_return_result(&cli.verbose, &cli.file, &mut raw_output).unwrap();

        if let Some(output) = raw_output.lines().next() {
            println!("{output}");
        }
    } else if cli.url != "none" {
        let mut raw_output = String::new();
        net::download_and_return_data(&cli.verbose, &cli.url, &mut raw_output).unwrap();

        if let Some(output) = raw_output.lines().next() {
            println!("{output}");
        }
    }
}
